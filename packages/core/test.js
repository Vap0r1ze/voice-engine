const VoiceEngine = require('.');

const core = VoiceEngine.start();

core.setVolumeChangeCallback((...args) => console.log(...args));
core.setLocalVolume(20);
