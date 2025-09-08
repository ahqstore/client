import { useEffect, useState } from "react";

interface Subcription {
  unregister: () => void;
}

export class Store<T> {
  private inner: T
  private listeners: {
    [key: number]: ((newData: T) => void)
  };
  private counter = 0;

  constructor(data: T) {
    this.inner = data
    this.listeners = {}
  }

  listen(listener: (newData: T) => void): Subcription {
    const counter = ++this.counter;

    this.listeners[counter] = listener;

    const unregister = () => {
      delete this.listeners[counter];
    };

    return {
      unregister
    };
  }

  get data(): T {
    return this.inner
  }

  set data(value: T) {
    this.inner = value;

    Object.values(this.listeners).forEach((a) => {
      try {
        a(value)
      } catch (e) {
        console.error("Unexpected: ");
        console.error(e);
      }
    });
  }

  set data_using_fn(call: (old: T) => T) {
    const value = call(this.inner);

    this.data = value;
  }
}

export function useStore<T>(store: Store<T>): T {
  const [data, setData] = useState<T>(store.data);

  useEffect(() => {
    const listenFn = (data: T) => {
      setData(data)
    };

    const sub = store.listen(listenFn);

    return () => {
      sub.unregister();
    };
  }, []);

  return data;
}