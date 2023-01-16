export type ImgData = {
    width: number;
    height: number;
    data: Uint8ClampedArray;
    pixelFormat: 'rgba'; // NOTE: There may be more pixel formats
};

export type Json<T> = string;

export type Callback<T extends any[] = []> = (cb: (...args: T) => void) => void;
