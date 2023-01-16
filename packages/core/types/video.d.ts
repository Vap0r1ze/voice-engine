export type VideoCodecName = 'H264' | 'VP8' | 'VP9';
export type VideoCodec = {
    name: VideoCodecName;
    type: number;
    rtxType: number;
    /** Codec params (eg: packetization-mode, level-asymmetry-allowed) */
    params: Record<string, string>;
};

export type VideoCodecCapabilities = {
    codec: VideoCodecName;
    encode: boolean;
    decode: boolean;
};
