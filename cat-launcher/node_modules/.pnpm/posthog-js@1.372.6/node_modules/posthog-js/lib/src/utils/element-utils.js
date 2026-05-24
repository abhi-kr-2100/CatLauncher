"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.isElementInToolbar = isElementInToolbar;
exports.isElementNode = isElementNode;
exports.isTag = isTag;
exports.isTextNode = isTextNode;
exports.isShadowRoot = isShadowRoot;
var constants_1 = require("../constants");
// Node.nodeType integer constants. We use the integers directly rather than
// `Node.ELEMENT_NODE` etc. for browser portability (IE11) and to avoid a
// runtime reference to the global `Node` in environments where it isn't defined.
var NODE_TYPE_ELEMENT = 1;
var NODE_TYPE_TEXT = 3;
var NODE_TYPE_DOCUMENT_FRAGMENT = 11;
function isElementInToolbar(el) {
    var _a;
    if (el instanceof Element) {
        // closest isn't available in IE11, but we'll polyfill when bundling
        return el.id === constants_1.TOOLBAR_ID || !!((_a = el.closest) === null || _a === void 0 ? void 0 : _a.call(el, '.' + constants_1.TOOLBAR_CONTAINER_CLASS));
    }
    return false;
}
/*
 * Check whether an element has nodeType Node.ELEMENT_NODE
 * @param {Element} el - element to check
 * @returns {boolean} whether el is of the correct nodeType
 */
function isElementNode(el) {
    return !!el && el.nodeType === NODE_TYPE_ELEMENT;
}
/*
 * Check whether an element is of a given tag type.
 * Due to potential reference discrepancies (such as the webcomponents.js polyfill),
 * we want to match tagNames instead of specific references because something like
 * element === document.body won't always work because element might not be a native
 * element.
 * @param {Element} el - element to check
 * @param {string} tag - tag name (e.g., "div")
 * @returns {boolean} whether el is of the given tag type
 */
function isTag(el, tag) {
    return !!el && !!el.tagName && el.tagName.toLowerCase() === tag.toLowerCase();
}
/*
 * Check whether an element has nodeType Node.TEXT_NODE
 * @param {Element} el - element to check
 * @returns {boolean} whether el is of the correct nodeType
 */
function isTextNode(el) {
    return !!el && el.nodeType === NODE_TYPE_TEXT;
}
/*
 * Check whether a node is a ShadowRoot — a DocumentFragment whose `host` is
 * a real Element. Plain DocumentFragments (e.g. <template> content) share
 * the same nodeType but have no `host`, so they are not ShadowRoots.
 * @param {Node|undefined|null} el - node to check
 * @returns {boolean} whether el is a ShadowRoot we can hop through to its host
 */
function isShadowRoot(el) {
    return !!el && el.nodeType === NODE_TYPE_DOCUMENT_FRAGMENT && isElementNode(el.host);
}
//# sourceMappingURL=element-utils.js.map