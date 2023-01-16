import { VoiceEngine as engine } from '..';

engine.setDeviceChangeCallback(() => {});
engine.setVolumeChangeCallback(() => {});
engine.setVideoInputInitializationCallback(() => {});
engine.setTransportOptions({
    ducking: false,
    idleJitterBufferFlush: true,
});
engine.getAudioSubsystem(() => {});
engine.getDebugLogging();
engine.setActiveSinksChangeCallback(() => {});
engine.getInputDevices(([firstDevice]) => {
    console.assert(firstDevice.name === 'default' && firstDevice.index === -1);
});
engine.getOutputDevices(([firstDevice]) => {
    console.assert(firstDevice.name === 'default' && firstDevice.index === -1);
});
engine.getVideoInputDevices(() => {}); // Cameras
engine.setInputDevice('default');
engine.setOutputDevice('default');
// engine.setVideoInputDevice('disabled');
// the above doesnt get called but it's the default state
engine.setInputVolume(1);
engine.setOutputVolume(1);
engine.setTransportOptions({ h264Enabled: true });
engine.setTransportOptions({ av1Enabled: true });
engine.setAecDump(true);
