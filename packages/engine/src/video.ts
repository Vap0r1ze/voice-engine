import * as engine from '@boisu/core';
import { FrameCallback, ImgData } from '@boisu/core/types/data';
import { popouts } from './globals';

type FrameSizeCallback = (width: number, height: number) => void;
const videoStreams: Record<string, Map<string, FrameCallback>> = {};
const directVideoStreams: Record<string, boolean> = {};

function getPopoutElementById(id: string) {
    for (const popout of popouts.values()) {
        const elem = popout.document?.getElementById(id);
        if (elem) return elem;
    }
    return null;
}

function requireContext(sinkId: string) {
    const canvas =
        document.getElementById(sinkId) ?? getPopoutElementById(sinkId);

    if (!(canvas instanceof HTMLCanvasElement)) return null;

    const ctx = canvas.getContext('2d');
    if (!ctx) console.warn('canvas context was %o for sink %o', ctx, sinkId);
    return ctx;
}

function addVideoOutputSinkInternal(
    sinkId: string,
    streamId: string,
    frameCallback: FrameCallback,
) {
    const sinks = (videoStreams[streamId] ??= new Map());

    sinks.set(sinkId, frameCallback);

    // NOTE(vap): if sink state isnt synced (populated) bad things will happen
    if (!sinks.size) return;

    console.log('Listening to frames for stream %o', streamId);
    const onFrame = (imgData: ImgData) => {
        const sinks = videoStreams[streamId];
        if (sinks) {
            for (const frameCallback of sinks.values())
                frameCallback?.(imgData);
        }
        engine.signalVideoOutputSinkReady(streamId);
    };
    engine.setVideoOutputSink(streamId, onFrame, true);
    notifyActiveSinksChange(streamId);
}

export function addVideoOutputSink(
    sinkId: string,
    streamId: string,
    frameSizeCallback: FrameSizeCallback,
) {
    let ctx: CanvasRenderingContext2D | null = null;
    addVideoOutputSinkInternal(sinkId, streamId, (imgData) => {
        if (!(ctx ??= requireContext(sinkId))) return;

        frameSizeCallback?.(imgData.width, imgData.height);
        ctx.putImageData(ctx.getImageData(0, 0, 1, 1), 0, 0);
    });
}

export function removeVideoOutputSink(sinkId: string, streamId: string) {
    const sinks = videoStreams[streamId];
    if (!sinks) return;

    sinks.delete(sinkId);
    if (sinks.size) return;

    delete videoStreams[streamId];
    console.log('Clearing frame listeners for stream %o', streamId);
    engine.clearVideoOutputSink(streamId);
    notifyActiveSinksChange(streamId);
}

let frameCount = 0;
export function getNextVideoOutputFrame(streamId: string) {
    const nextFrameSinkId = `videoFrame-${++frameCount}`;

    return new Promise<ImgData>((resolve, reject) => {
        setTimeout(() => {
            removeVideoOutputSink(nextFrameSinkId, streamId);
            reject(new Error('getNextVideoOutputFrame timed out'));
        }, 5000);

        addVideoOutputSinkInternal(nextFrameSinkId, streamId, (imageData) => {
            removeVideoOutputSink(nextFrameSinkId, streamId);
            resolve({
                width: imageData.width,
                height: imageData.height,
                data: new Uint8ClampedArray(imageData.data.buffer),
            });
        });
    });
}

type OnActiveSinksChange = (streamId: string, hasVideoSink: boolean) => void;
let onActiveSinksChange: OnActiveSinksChange;
export function setActiveSinksChangeCallback(onChange: OnActiveSinksChange) {
    onActiveSinksChange = onChange;
}
export function notifyActiveSinksChange(streamId: string) {
    onActiveSinksChange?.(
        streamId,
        !!videoStreams[streamId].size || !!directVideoStreams[streamId],
    );
}

export function addDirectVideoOutputSink(streamId: string) {
    console.log('Adding sink for direct stream %o', streamId);
    engine.addDirectVideoOutputSink(streamId);
    directVideoStreams[streamId] = true;
    notifyActiveSinksChange(streamId);
}
export function removeDirectVideoOutputSink(streamId: string) {
    console.log('Removing sink for direct stream %o', streamId);
    engine.removeDirectVideoOutputSink(streamId);
    delete directVideoStreams[streamId];
    notifyActiveSinksChange(streamId);
}
