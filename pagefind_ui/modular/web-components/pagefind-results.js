import { PagefindElement } from "./base-element.js";
import El from "../helpers/element-builder.js";

const templateNodes = (templateResult) => {
    if (templateResult instanceof Element) {
        return [templateResult];
    }
    if (Array.isArray(templateResult) && templateResult.every(r => r instanceof Element)) {
        return templateResult;
    }
    if (typeof templateResult === "string" || templateResult instanceof String) {
        const wrap = document.createElement("div");
        wrap.innerHTML = templateResult;
        return [...wrap.childNodes];
    }
    console.error(`[Pagefind Results]: Expected template to return HTML element or string, got ${typeof templateResult}`);
    return [];
};

const placeholderTemplate = () => {
    const placeholder = (max = 30) => ". ".repeat(Math.floor(10 + Math.random() * max));
    return `<li class="pagefind-modular-list-result">
    <div class="pagefind-modular-list-thumb" data-pfmod-loading></div>
    <div class="pagefind-modular-list-inner">
        <p class="pagefind-modular-list-title" data-pfmod-loading>${placeholder(30)}</p>
        <p class="pagefind-modular-list-excerpt" data-pfmod-loading>${placeholder(40)}</p>
    </div>
</li>`;
};

const resultTemplate = (result, showImages) => {
    const wrapper = new El("li")
        .class("pagefind-modular-list-result")
        .attrs({ role: "option" });

    if (showImages && result?.meta?.image) {
        const thumb = new El("div").class("pagefind-modular-list-thumb").addTo(wrapper);
        new El("img").class("pagefind-modular-list-image").attrs({
            src: result.meta.image,
            alt: result.meta.image_alt || result.meta.title
        }).addTo(thumb);
    }

    const inner = new El("div").class("pagefind-modular-list-inner").addTo(wrapper);
    const title = new El("p").class("pagefind-modular-list-title").addTo(inner);
    new El("a").class("pagefind-modular-list-link")
        .text(result.meta?.title)
        .attrs({ href: result.meta?.url || result.url })
        .addTo(title);

    new El("p").class("pagefind-modular-list-excerpt").html(result.excerpt).addTo(inner);

    return wrapper.element;
};

const nearestScrollParent = (el) => {
    if (!(el instanceof HTMLElement)) return null;
    const overflowY = window.getComputedStyle(el).overflowY;
    const isScrollable = overflowY !== 'visible' && overflowY !== 'hidden';
    return isScrollable ? el : nearestScrollParent(el.parentNode);
};

class Result {
    constructor(opts = {}) {
        this.rawResult = opts.result;
        this.placeholderNodes = opts.placeholderNodes;
        this.resultFn = opts.resultFn;
        this.intersectionEl = opts.intersectionEl;
        this.showImages = opts.showImages;
        this.result = null;
        this.waitForIntersection();
    }

    waitForIntersection() {
        if (!this.placeholderNodes?.length) return;

        const options = {
            root: this.intersectionEl,
            rootMargin: "0px",
            threshold: 0.01,
        };

        const observer = new IntersectionObserver((entries, obs) => {
            if (this.result !== null) return;
            if (entries?.[0]?.isIntersecting) {
                this.load();
                obs.disconnect();
            }
        }, options);

        observer.observe(this.placeholderNodes[0]);
    }

    async load() {
        if (!this.placeholderNodes?.length) return;

        this.result = await this.rawResult.data();
        const resultTemplate = this.resultFn(this.result, this.showImages);
        const resultNodes = templateNodes(resultTemplate);

        while (this.placeholderNodes.length > 1) {
            this.placeholderNodes.pop().remove();
        }

        this.placeholderNodes[0].replaceWith(...resultNodes);
    }
}

export class PagefindResults extends PagefindElement {
    static get observedAttributes() {
        return ['show-images', 'page-size', 'show-sub-results', 'excerpt-length'];
    }

    constructor() {
        super();
        this.containerEl = null;
        this.intersectionEl = document.body;
        this.results = [];
        this.showImages = true;
        this.pageSize = null;
        this.showSubResults = false;
        this.excerptLength = null;
        this.placeholderTemplate = placeholderTemplate;
        this.resultTemplate = resultTemplate;
        this.warnedFields = new Set();
    }

    init() {
        if (this.hasAttribute('show-images')) {
            this.showImages = this.getAttribute('show-images') !== 'false';
        }
        if (this.hasAttribute('page-size')) {
            this.pageSize = parseInt(this.getAttribute('page-size'), 10) || null;
        }
        if (this.hasAttribute('show-sub-results')) {
            this.showSubResults = this.getAttribute('show-sub-results') !== 'false';
        }
        if (this.hasAttribute('excerpt-length')) {
            this.excerptLength = parseInt(this.getAttribute('excerpt-length'), 10) || null;
        }

        this.checkForTemplates();
        this.render();
    }

    checkForTemplates() {
        let resultTemplateEl = this.querySelector('template[data-template="result"]');
        if (!resultTemplateEl) {
            resultTemplateEl = this.querySelector('template:not([data-template])');
        }

        if (resultTemplateEl) {
            this.customResultTemplate = resultTemplateEl;
            this.resultTemplate = (result, showImages) => this.processResultTemplate(result, showImages);
        }

        const placeholderTemplateEl = this.querySelector('template[data-template="placeholder"]');
        if (placeholderTemplateEl) {
            this.customPlaceholderTemplate = placeholderTemplateEl;
            this.placeholderTemplate = () => this.processPlaceholderTemplate();
        }
    }

    processResultTemplate(result, showImages) {
        if (!this.customResultTemplate) {
            return resultTemplate(result, showImages);
        }

        const clone = this.customResultTemplate.content.cloneNode(true);
        const BLOCKED_ATTRIBUTES = /^on[a-z]+$/i;
        const attributesToRemove = [];

        clone.querySelectorAll('*').forEach(el => {
            Array.from(el.attributes).forEach(attr => {
                if (!attr.name.startsWith('pagefind-')) return;

                const directiveName = attr.name.substring('pagefind-'.length);
                const fieldName = attr.value;

                attributesToRemove.push({ el, attrName: attr.name });

                if (BLOCKED_ATTRIBUTES.test(directiveName)) {
                    console.warn(`[Pagefind]: Blocked unsafe attribute directive: pagefind-${directiveName}`);
                    return;
                }

                const value = this.getResultField(result, fieldName);

                if (value === undefined || value === null) {
                    if (!this.warnedFields.has(fieldName)) {
                        console.warn(`[Pagefind]: Field "${fieldName}" not found in result object`);
                        this.warnedFields.add(fieldName);
                    }
                    return;
                }

                if (directiveName === 'text') {
                    el.textContent = value;
                } else if (directiveName === 'html') {
                    el.innerHTML = value;
                } else {
                    // All other directives set attributes (href, src, alt, data-*, etc.)
                    if ((directiveName === 'href' || directiveName === 'src') && /^\s*javascript:/i.test(value)) {
                        console.warn(`[Pagefind]: Blocked unsafe ${directiveName} value:`, value);
                        return;
                    }
                    el.setAttribute(directiveName, value);
                }
            });
        });

        attributesToRemove.forEach(({ el, attrName }) => el.removeAttribute(attrName));

        const wrapper = clone.querySelector('li');
        if (wrapper) {
            wrapper.setAttribute('role', 'option');
            return wrapper;
        }

        const li = document.createElement('li');
        li.setAttribute('role', 'option');
        li.appendChild(clone);
        return li;
    }

    processPlaceholderTemplate() {
        if (!this.customPlaceholderTemplate) {
            return placeholderTemplate();
        }

        const clone = this.customPlaceholderTemplate.content.cloneNode(true);
        const wrapper = clone.querySelector('li');

        if (wrapper) return wrapper;

        const li = document.createElement('li');
        li.appendChild(clone);
        return li;
    }

    getResultField(result, field) {
        // Try meta first, then top-level
        if (result.meta?.[field] !== undefined) {
            return result.meta[field];
        }
        if (result[field] !== undefined) {
            return result[field];
        }
        return null;
    }

    render() {
        const savedTemplates = [];
        this.querySelectorAll('template').forEach(t => savedTemplates.push(t));

        this.innerHTML = "";

        savedTemplates.forEach(t => {
            t.style.display = 'none';
            this.appendChild(t);
        });

        this.containerEl = new El("ul")
            .class("pagefind-modular-results")
            .attrs({
                role: "listbox",
                "aria-label": "Search results"
            })
            .addTo(this);
    }

    append(nodes) {
        for (const node of nodes) {
            this.containerEl.appendChild(node);
        }
    }

    register(instance) {
        instance.on("results", (results) => {
            if (!this.containerEl) return;

            this.containerEl.innerHTML = "";
            this.warnedFields.clear();
            this.intersectionEl = nearestScrollParent(this.containerEl);

            this.results = results.results.map(r => {
                const placeholderNodes = templateNodes(this.placeholderTemplate());
                this.append(placeholderNodes);
                return new Result({
                    result: r,
                    placeholderNodes,
                    resultFn: this.resultTemplate,
                    intersectionEl: this.intersectionEl,
                    showImages: this.showImages
                });
            });
        });

        instance.on("loading", () => {
            if (!this.containerEl) return;
            this.containerEl.innerHTML = "";
        });

        instance.on("error", (error) => {
            this.showError({
                message: error.message || "Failed to load search results",
                details: error.bundlePath ? `Bundle path: ${error.bundlePath}` : null
            });
        });
    }

    setupAria() {
        this.ensureId("pagefind-results");

        const input = this.findRelatedComponent('pagefind-input');

        if (input?.inputEl && this.instance) {
            this.instance.on("results", (searchResults) => {
                if (!input.inputEl) return;
                const hasResults = searchResults?.results?.length > 0;
                input.inputEl.setAttribute('aria-expanded', hasResults ? 'true' : 'false');
            });
        }
    }

    update() {
        this.render();
        if (this.instance) {
            this.register(this.instance);
        }
    }
}

if (!customElements.get('pagefind-results')) {
    customElements.define('pagefind-results', PagefindResults);
}
