import * as core from '..';

const streamParams = [
    {
        type: 'video',
        active: false,
        rid: '50',
        ssrc: 123401,
        rtxSsrc: 123402,
        quality: 50,
        maxBitrate: 2500000,
        maxFrameRate: 20,
        maxResolution: {
            type: 'fixed',
            width: 1280,
            height: 720,
        },
    },
    {
        type: 'video',
        active: false,
        rid: '100',
        ssrc: 123403,
        rtxSsrc: 123404,
        quality: 100,
        maxBitrate: 2500000,
        maxFrameRate: 20,
        maxResolution: {
            type: 'fixed',
            width: 1280,
            height: 720,
        },
    },
] as const;
const conn = core.createVoiceConnection(
    '123456789012345678',
    {
        ssrc: 123400,
        address: '1.2.3.4',
        port: 12345,
        modes: [
            'aead_aes256_gcm_rtpsize',
            'aead_aes256_gcm',
            'aead_xchacha20_poly1305_rtpsize',
            'xsalsa20_poly1305_lite_rtpsize',
            'xsalsa20_poly1305_lite',
            'xsalsa20_poly1305_suffix',
            'xsalsa20_poly1305',
        ],
        experiments: ['fixed_keyframe_interval'],
        qosEnabled: false,
        streamParameters: streamParams,
    },
    (_, info) => console.log('new conn', info),
);

// This is for screenshares
conn.setDesktopSourceStatusCallback((status) =>
    console.log('desktop status', status),
);

// Insane option spam
conn.setTransportOptions({
    inputMode: 1,
    inputModeOptions: {
        vadThreshold: -100,
        vadAutoThreshold: -1,
        vadUseKrisp: true,
        vadLeading: 5,
        vadTrailing: 25,
    },
});
conn.setTransportOptions({ remoteSinkWantsMaxFramerate: 20 });
core.setTransportOptions({ echoCancellation: false });
core.setTransportOptions({ noiseSuppression: false });
core.setTransportOptions({ automaticGainControl: true });
core.setTransportOptions({ noiseCancellation: true });
conn.setTransportOptions({
    attenuation: false,
    attenuationFactor: 1,
    attenuateWhileSpeakingSelf: false,
    attenuateWhileSpeakingOthers: true,
});
conn.setTransportOptions({ qos: false });
conn.setTransportOptions({ experimentalEncoders: false });
conn.setTransportOptions({ hardwareH264: true });
conn.setTransportOptions({ encodingVoiceBitRate: 140000 });
conn.setTransportOptions({
    encodingVideoWidth: 1280,
    encodingVideoHeight: 720,
    encodingVideoFrameRate: 30,
    remoteSinkWantsPixelCount: 921600,
    encodingVideoBitRate: 2500000,
    encodingVideoMinBitRate: 150000,
    encodingVideoMaxBitRate: 2500000,
    streamParameters: streamParams.map((stream) => ({
        ...stream,
        maxPixelCount: 921600,
    })),
});

conn.setSelfMute(true); // same as conn.setTransportOptions({ selfMute: true })
conn.setSelfDeafen(false);
conn.setLocalVolume('ven id', 0); // user "ven id" 0% volume (naturally)
conn.setLocalVolume('vap id', 2); // user "vap id" 200% volume (so loud)

// await createVoiceConnectionWithOptions callback

core.getCodecCapabilities((capabilities) => console.log('caps', capabilities));

// await getCodecCapabilities callback

conn.getEncryptionModes((modes) => console.log('enc modes', modes));

// await getEncryptionModes callback

conn.setTransportOptions({
    minimumJitterBufferLevel: 0,
    postponeDecodeLevel: 75,
    fec: true,
    packetLossRate: 0.3,
    prioritySpeakerDucking: 0.1,
    callBitRate: 600000,
    callMinBitRate: 150000,
    callMaxBitRate: 10000000,
    encodingVideoDegradationPreference: 2,
    reconnectInterval: 60000,
    userChannelIds: {
        userId: '123456789012345678',
        channelId: '123456789012345678',
        guildId: '123456789012345678',
    },
});

conn.setOnSpeakingCallback((user, flags) =>
    console.log('speaking %o %o', user, flags),
);
conn.setOnSpeakingWhileMutedCallback(() =>
    console.log('self speaking while muted'),
);
conn.setPingInterval(5000);
conn.setPingTimeoutCallback(() => console.log('ping timeout'));
// Things like this must be adjustable mid-connection
core.setTransportOptions({
    builtInEchoCancellation: true,
    echoCancellation: false,
    noiseSuppression: false,
    automaticGainControl: true,
    noiseCancellation: true,
});
core.setNoInputThreshold(-100); // will always be -100 i think
core.setNoInputCallback((crosses) =>
    console.log(crosses ? 'yay input has noise' : 'input is silent?!?!'),
);

conn.setOnVideoCallback((user, ssrc, sink, sources) => {
    if (ssrc === 0) console.log('local camera init', sources);
    else
        console.log('user "%s" turned on their camera (ew)', {
            ssrc,
            sink,
            sources,
        });
});
conn.setOnDesktopSourceEnded(() =>
    console.log('local user closed what they were sharing'),
);
conn.setOnSoundshare((success) =>
    console.log(
        'eavesdropping on ur screen audio now, idk why this would be false:',
        success,
    ),
);
conn.setOnSoundshareEnded(() => {});
conn.setOnSoundshareFailed(console.log.bind(console, 'a'));
conn.mergeUsers([]);

// await all(setOnSpeakingCallback, setNoInputCallback, setOnVideoCallback)

conn.setTransportOptions({
    videoEncoder: {
        name: 'H264',
        type: 101,
        rtxType: 102,
        params: {
            'level-asymmetry-allowed': '1',
            'packetization-mode': '1',
            'profile-level-id': '4d0033',
            'hardware-h264': '1',
        },
    },
    videoDecoders: [
        {
            name: 'H264',
            type: 101,
            rtxType: 102,
            params: {
                'level-asymmetry-allowed': '1',
                'packetization-mode': '1',
                'profile-level-id': '42e034',
                'hardware-h264': '1',
            },
        },
        {
            name: 'VP8',
            type: 103,
            rtxType: 104,
            params: {},
        },
        {
            name: 'VP9',
            type: 105,
            rtxType: 106,
            params: {},
        },
    ],
    audioEncoder: {
        type: 120,
        name: 'opus',
        freq: 48000,
        pacsize: 960,
        channels: 1,
        rate: 64000,
    },
    audioDecoders: [
        {
            type: 120,
            name: 'opus',
            freq: 48000,
            channels: 2,
            params: {
                stereo: '1',
            },
        },
    ],
});
conn.setTransportOptions({
    encryptionSettings: {
        mode: 'aead_aes256_gcm_rtpsize',
        secretKey: Array(32).fill(0),
    },
});
conn.setTransportOptions({
    remoteSinkWantsPixelCount: 921600,
    remoteSinkWantsMaxFramerate: 20,
    encodingVideoMinBitRate: 150000,
    encodingVideoMaxBitRate: 2500000,
    encodingVideoBitRate: 2500000,
    streamParameters: [
        {
            type: 'video',
            active: false,
            rid: '50',
            ssrc: 445099,
            rtxSsrc: 445100,
            quality: 50,
            maxBitrate: 2500000,
            maxFrameRate: 20,
            maxResolution: {
                type: 'fixed',
                width: 1280,
                height: 720,
            },
            maxPixelCount: 921600,
        },
        {
            type: 'video',
            active: false,
            rid: '100',
            ssrc: 445101,
            rtxSsrc: 445102,
            quality: 100,
            maxBitrate: 2500000,
            maxFrameRate: 20,
            maxResolution: {
                type: 'fixed',
                width: 1280,
                height: 720,
            },
            maxPixelCount: 921600,
        },
    ],
});
