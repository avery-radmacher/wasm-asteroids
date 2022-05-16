'use strict';
let Module = {};

// #region copying strings
let utfDecoder = new TextDecoder('utf-8');
let getStr = function (module, ptr, len) {
  let slice = new Uint8Array(module.memory.buffer, ptr, len);
  return utfDecoder.decode(slice);
};

let utfEncoder = new TextEncoder('utf-8');
let putStr = function (module, str) {
  let buf = utfEncoder.encode(str);
  let ptr = module.alloc(buf.length);
  let slice = new Uint8Array(module.memory.buffer, ptr, buf.length);
  slice.set(buf);
  return { ptr: ptr, len: buf.length };
};
// #endregion

let io = {
  puts: (ptr, len) => console.log(getStr(Module, ptr, len)),
};

let time = {
  performance_now: () => performance.now(),
};

let eventLoop = function (Module) {
  const EVENT_ANIMATION_FRAME = 0;
  const EVENT_KEY_DOWN = 1;
  const EVENT_KEY_UP = 2;
  let eventLoopsDict = new Map();
  eventLoopsDict.counter = 0;

  let keyEventFlags = function (event) {
    return (event.shiftKey ? 1 : 0) | (event.ctrlKey ? 2 : 0) | (event.altKey ? 4 : 0);
  };
  let charKey = function (event) {
    if (event.key.length !== 1) {
      return 0xffffffff;
    }
    return event.key.charCodeAt(0);
  };

  let EventLoop = class {
    constructor(callback) {
      let self = this;
      let id = ++eventLoopsDict.counter;
      eventLoopsDict.set(id, self);
      self.id = id;
      self.dead = false;
      self.rafId = null;

      self.rafCb = function () {
        self.rafId = null;
        if (self.dead) {
          return;
        }
        callback(id, EVENT_ANIMATION_FRAME, 0, 0, 0);
      };

      self.keyDown = function (event) {
        if (self.dead) {
          return;
        }
        callback(id, EVENT_KEY_DOWN, event.which, charKey(event), keyEventFlags(event));
      };

      self.keyUp = function (event) {
        if (self.dead) {
          return;
        }
        callback(id, EVENT_KEY_UP, event.which, charKey(event), keyEventFlags(event));
      };

      self.subscribeKeyboard();
    }

    raf() {
      let self = this;
      if (self.dead) {
        return;
      }
      self.rafId = requestAnimationFrame(self.rafCb);
    }

    subscribeKeyboard() {
      let self = this;
      if (self.dead) {
        return;
      }
      window.addEventListener('keydown', self.keyDown);
      window.addEventListener('keyup', self.keyUp);
    }
  };

  let raf = function (id) {
    if (!eventLoopsDict.has(id)) {
      return false;
    }
    eventLoopsDict.get(id).raf();
    return true;
  };

  return {
    event_loop_new: () => new EventLoop(Module.event_loop_cb).id,
    event_loop_raf: raf,
  };
};

let svg = {
  svg_set_path: (ptr, len) => window.path.setAttributeNS(null, 'd', getStr(Module, ptr, len)),
};

let randSource = (module) => (ptr, len) => {
  const OK = 0;
  const RANGE_ERROR = 1;
  const QUOTA_ERROR = 2;

  try {
    let slice = new Uint8Array(module.memory.buffer, ptr, len);
    window.crypto.getRandomValues(slice);
  } catch (e) {
    if (e instanceof RangeError) {
      return RANGE_ERROR;
    } else if (e instanceof DOMException && x.code === DOMException.QUOTA_EXCEEDED_ERR) {
      return QUOTA_ERROR;
    } else {
      throw e;
    }
  }

  return OK;
};

let rand = {
  js_fill_rand: randSource(Module),
};

let imports = {
  env: Object.assign({}, time, eventLoop(Module), io, svg, rand),
};

fetch('/target/wasm32-unknown-unknown/release/svg_asteroids.wasm')
  .then((response) => response.arrayBuffer())
  .then((bytes) => WebAssembly.instantiate(bytes, imports))
  .then((results) => {
    let instance = results.instance;
    let exports = instance.exports;
    Object.assign(Module, {
      alloc: exports.alloc,
      memory: exports.memory,
      event_loop_cb: exports.event_loop_cb,
    });
    exports.my_main();
  });
