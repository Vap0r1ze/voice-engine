export type FrameCallback = (imgData: ImgData) => void;
export type ImgData = {
    width: number;
    height: number;
    data: Uint8ClampedArray;
};

export type Json<T> = string & { ['\u2728']: T };

declare global {
    interface JSON {
        parse<D extends Json<any>>(data: D): D['\u2728'];
    }
}

export type Callback<T extends any[] = []> = (cb: (...args: T) => void) => void;
