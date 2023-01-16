import { VoiceEngine } from 'voice-engine-native';
import { Manager } from './manager';

export function createVoiceEngine(manager: Manager) {
    return VoiceEngine;
}
