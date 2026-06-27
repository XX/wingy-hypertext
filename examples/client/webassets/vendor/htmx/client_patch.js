
export function init_htmx_request_interception(wasm) {
    const _getAllResponseHeaders = XMLHttpRequest.prototype.getAllResponseHeaders;
    const _getResponseHeader = XMLHttpRequest.prototype.getResponseHeader;
    const _open = XMLHttpRequest.prototype.open;
    const _send = XMLHttpRequest.prototype.send;

    XMLHttpRequest.prototype.open = function(method, url, ...args) {
        this._intercepted = false;
        this._url = url;
        return _open.call(this, method, url, ...args);
    };

    XMLHttpRequest.prototype.send = function(body) {
        if (shouldIntercept(this._url)) {
            this._intercepted = true;
            this._mockedResponse = wasm.request(this._url);

            setTimeout(() => {
                Object.defineProperty(this, "readyState",   { get: () => 4, configurable: true });
                Object.defineProperty(this, "status",       { get: () => 200, configurable: true });
                Object.defineProperty(this, "responseText", { get: () => this._mockedResponse, configurable: true });
                Object.defineProperty(this, "response",     { get: () => this._mockedResponse, configurable: true });
                this.onload && this.onload();
            }, 0);
            return;
        }
        return _send.call(this, body);
    };

    XMLHttpRequest.prototype.getAllResponseHeaders = function() {
        if (this._intercepted) return "";
        return _getAllResponseHeaders.call(this);
    };

    XMLHttpRequest.prototype.getResponseHeader = function(name) {
        if (this._intercepted) return null;
        return _getResponseHeader.call(this, name);
    };
}

function shouldIntercept(url) {
    return !url.startsWith("/api/")
}

export { init_htmx_request_interception as default };
