function includes(str, needle) {
    return -1 !== str.indexOf(needle);
}
const trim = function(str) {
    return str.trim();
};
const stripLeadingDollar = function(s) {
    return s.replace(/^\$/, '');
};
function isDistinctIdStringLike(value) {
    return [
        'distinct_id',
        'distinctid'
    ].includes(value.toLowerCase());
}
function deepSortKeys(value) {
    if (null === value || 'object' != typeof value) return value;
    if (Array.isArray(value)) return value.map(deepSortKeys);
    return Object.keys(value).sort().reduce((acc, key)=>{
        acc[key] = deepSortKeys(value[key]);
        return acc;
    }, {});
}
function getPersonPropertiesHash(distinct_id, userPropertiesToSet, userPropertiesToSetOnce) {
    return JSON.stringify({
        distinct_id,
        userPropertiesToSet: userPropertiesToSet ? deepSortKeys(userPropertiesToSet) : void 0,
        userPropertiesToSetOnce: userPropertiesToSetOnce ? deepSortKeys(userPropertiesToSetOnce) : void 0
    });
}
export { getPersonPropertiesHash, includes, isDistinctIdStringLike, stripLeadingDollar, trim };
