import type {
  TranslationStrings,
  TranslationFile,
  TextDirection,
} from "../types";

// @ts-expect-error - glob import handled by esbuild-plugin-import-glob
import * as translationFiles from "../../translations/*.json";
import { parse as parseBCP47 } from "bcp-47";

const translations: Record<string, TranslationStrings> = {};
const filenames = translationFiles.filenames as string[];
const contents = translationFiles.default as TranslationFile[];

for (let i = 0; i < filenames.length; i++) {
  const match = filenames[i].match(/([^\/]+)\.json$/);
  if (!match) continue;
  const lang = match[1];
  translations[lang] = {
    language: lang,
    direction: contents[i].direction || "ltr",
    ...contents[i].strings,
  };
}

/**
 * Get translations for a given language code.
 * Uses BCP-47 parsing with fallback chain:
 * 1. language-script-region (e.g., zh-Hans-CN)
 * 2. language-region (e.g., en-US)
 * 3. language (e.g., en)
 * 4. English fallback
 */
export function getTranslations(langCode?: string): TranslationStrings {
  if (!langCode) {
    return translations["en"];
  }

  const parsed = parseBCP47(langCode.toLowerCase());

  const keys: string[] = [];
  if (parsed.language && parsed.script && parsed.region) {
    keys.push(`${parsed.language}-${parsed.script}-${parsed.region}`);
  }
  if (parsed.language && parsed.region) {
    keys.push(`${parsed.language}-${parsed.region}`);
  }
  if (parsed.language) {
    keys.push(parsed.language);
  }

  for (const key of keys) {
    if (translations[key]) {
      return translations[key];
    }
  }

  return translations["en"];
}

/**
 * Interpolate placeholders in a translation string.
 * Placeholders are in the format [PLACEHOLDER_NAME].
 */
export function interpolate(
  str: string | undefined,
  replacements: Record<string, string | number> = {},
): string {
  if (!str) return "";

  let result = str;
  for (const [placeholder, value] of Object.entries(replacements)) {
    result = result.replace(
      new RegExp(`\\[${placeholder}\\]`, "g"),
      String(value),
    );
  }
  return result;
}
