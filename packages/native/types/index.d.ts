export { VoiceEngine } from './engine'

declare module 'voice-engine-native' {
  export const VoiceEngine: VoiceEngine
}
