declare function elementIsVisible(element: HTMLElement, cache: WeakMap<HTMLElement, boolean>): boolean;
interface InferredSelector {
    autoData: string;
    text: string | null;
    excludeText?: boolean;
    precision?: number;
}
/**
 * if inferSelector is the sauce, this is the nugget
 *
 * find an element in the dom using the element inference data
 *
 * 1. try each group of selectors, starting with most specific (lowest cardinality)
 * 2. try each selector in the group - run the css query, go to offset
 * 3. "vote" for the element if it was found
 * 4. return early if any element gets majority votes
 * 5. return element w/ most votes
 */
declare function findElement(selector: InferredSelector): HTMLElement | null;
declare function getElementPath(el: HTMLElement | null, depth?: number): string | null;

export { elementIsVisible, findElement, getElementPath };
