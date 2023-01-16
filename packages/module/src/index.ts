import * as engineCore from '@boisu/core';
import { DegradationPreference } from '@boisu/core/types/transport';
import { FeatureFlag, Manager } from './manager';
import {
    addVideoOutputSink,
    removeVideoOutputSink,
    getNextVideoOutputFrame,
    setActiveSinksChangeCallback,
    notifyActiveSinksChange,
    addDirectVideoOutputSink,
    removeDirectVideoOutputSink,
} from './video';

export function createVoiceEngine(manager: Manager) {
    manager.features.declare(FeatureFlag.VoicePanning);
    manager.features.declare(FeatureFlag.SetAudioDeviceById);
    manager.features.declare(FeatureFlag.SetVideoDeviceById);

    return {
        ...engineCore,

        DegradationPreference,

        createOwnStreamConnectionWithOptions:
            engineCore.createVoiceConnectionWithOptions,

        // Video
        addVideoOutputSink,
        removeVideoOutputSink,
        getNextVideoOutputFrame,
        setActiveSinksChangeCallback,
        notifyActiveSinksChange,
        addDirectVideoOutputSink,
        removeDirectVideoOutputSink,
    };
}
