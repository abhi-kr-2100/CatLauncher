function countBy(map, mapper) {
    const result = new Map();
    for (const [key, value] of map) {
        const mappedKey = mapper(value, key, map);
        result.set(mappedKey, (result.get(mappedKey) ?? 0) + 1);
    }
    return result;
}

export { countBy };
