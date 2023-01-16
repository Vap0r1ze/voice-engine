import { VADOptions } from './audio';

export type AudioSubsystem = 'standard' | 'legacy' | 'experimental';

export type EngineTransportOptions = VADOptions &
    Partial<{
        h264Enabled: boolean;
        av1Enabled: boolean;

        ducking: boolean;
        idleJitterBufferFlush: boolean;
    }>;
