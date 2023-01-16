import { ImgData } from './data';

export type CameraDevice = {
    guid: string;
    name: string;
    index: number;
    facing: 'front' | 'back' | 'unknown';
};

export enum MediaFilterType {
    None = '',
    BackgroundBlur = 'background_blur',
    BackgroundReplacement = 'background_replacement',
}
export type MediaFilterSettings = {
    cameraBackgroundPreview?: MediaFilter;
    cameraBackgroundLive?: MediaFilter;
};
export type MediaFilter =
    | {
          graph: MediaFilterType.None; // No filter
          target: MediaFilterTarget;
      }
    | {
          graph: MediaFilterType.BackgroundBlur;
          target: MediaFilterTarget;
      }
    | {
          graph: MediaFilterType.BackgroundReplacement;
          target: MediaFilterTarget;
          /** Used for bitmap backgrounds */
          image: ImgData;
      }
    | {
          graph: MediaFilterType.BackgroundReplacement;
          target: MediaFilterTarget;
          /** Used for mp4 backgrounds */
          blob: Uint8ClampedArray;
      };
export type MediaFilterTarget =
    | {
          type: 'input_device';
      }
    | {
          type: 'stream'; // TODO: figure out what streams are
          streamId: string;
      };
