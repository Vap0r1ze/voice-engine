import * as core from '..';

core.setDeviceChangeCallback(() => {});
core.setVolumeChangeCallback(() => {});
core.setVideoInputInitializationCallback(() => {});
core.setTransportOptions({
    ducking: false,
    idleJitterBufferFlush: true,
});
core.getAudioSubsystem(() => {});
core.getDebugLogging();
core.getInputDevices(([firstDevice]) => {
    console.assert(firstDevice.name === 'default' && firstDevice.index === -1);
});
core.getOutputDevices(([firstDevice]) => {
    console.assert(firstDevice.name === 'default' && firstDevice.index === -1);
});
core.getVideoInputDevices(() => {}); // Cameras
core.setInputDevice('default');
core.setOutputDevice('default');
// core.setVideoInputDevice('disabled');
// the above doesnt get called but it's the default state
core.setInputVolume(1);
core.setOutputVolume(1);
core.setTransportOptions({ h264Enabled: true });
core.setTransportOptions({ av1Enabled: true });
core.setAecDump(true);
