import { memory, event_loop_cb } from '../../../index_bg.wasm';

export const { event_loop_new, event_loop_raf } = (() => {
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
    event_loop_new: () => new EventLoop(event_loop_cb).id,
    event_loop_raf: raf,
  };
})();

export const svg_set_path = (str) => window.path.setAttributeNS(null, 'd', str);

export const js_fill_rand = (ptr, len) => {
  const OK = 0;
  const RANGE_ERROR = 1;
  const QUOTA_ERROR = 2;

  try {
    let slice = new Uint8Array(memory.buffer, ptr, len);
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
