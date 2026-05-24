function forEach(set, callback) {
    for (const value of set) {
        callback(value, value, set);
    }
}

export { forEach };
