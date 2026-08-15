import test from "ava";
import type * as external from "../types/index";

import { calculate_sub_results } from "../lib/sub_results";

test("explicit anchor titles create sub results", (t) => {
  const fragment = {
    url: "/guide/",
    raw_content: "Install the package",
    meta: { title: "Guide" },
    anchors: [
      {
        element: "section",
        id: "installation",
        title: "Installation",
        location: 0,
      },
    ],
    weighted_locations: [{ weight: 1, balanced_score: 1, location: 1 }],
  } as PagefindSearchFragment;

  const results = calculate_sub_results(fragment, 30);

  t.is(results.length, 1);
  t.is(results[0].title, "Installation");
  t.is(results[0].url, "/guide/#installation");
});
