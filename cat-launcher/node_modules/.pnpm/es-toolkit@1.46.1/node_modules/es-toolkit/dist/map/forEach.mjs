function forEach(map, callback) {
    for (const [key, value] of map) {
        callback(value, key, map);
    }
}

export { forEach };
