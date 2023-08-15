pub const SOUND_TAG_BOILERPLATE: &str = r#"
    <sound
        name="{}"
        polyphonic="auto"
        voicePriority="1"
        sideChainSend="2147483647"
        mode="subtractive"
        lpfMode="24dB"
        modFXType="none">
        <osc1
            type="sample"
            loopMode="1"
            reversed="0"
            timeStretchEnable="0"
            timeStretchAmount="0"
            fileName="{}">
            <zone
                startSamplePos="0"
                endSamplePos="52500" />
        </osc1>
        <osc2
            type="sample"
            loopMode="0"
            reversed="0"
            timeStretchEnable="0"
            timeStretchAmount="0">
        </osc2>
        <lfo1 type="triangle" syncLevel="0" />
        <lfo2 type="triangle" />
        <unison num="1" detune="8" />
        <delay
            pingPong="1"
            analog="0"
            syncLevel="7" />
        <compressor
            syncLevel="6"
            attack="327244"
            release="936" />
        <defaultParams
            arpeggiatorGate="0x00000000"
            portamento="0x80000000"
            compressorShape="0xDC28F5B2"
            oscAVolume="0x7FFFFFFF"
            oscAPulseWidth="0x00000000"
            oscAWavetablePosition="0x00000000"
            oscBVolume="0x80000000"
            oscBPulseWidth="0x00000000"
            oscBWavetablePosition="0x00000000"
            noiseVolume="0x80000000"
            volume="0x4CCCCCA8"
            pan="0x00000000"
            lpfFrequency="0x7FFFFFFF"
            lpfResonance="0x80000000"
            hpfFrequency="0x80000000"
            hpfResonance="0x80000000"
            lfo1Rate="0x1999997E"
            lfo2Rate="0x00000000"
            modulator1Amount="0x80000000"
            modulator1Feedback="0x80000000"
            modulator2Amount="0x80000000"
            modulator2Feedback="0x80000000"
            carrier1Feedback="0x80000000"
            carrier2Feedback="0x80000000"
            modFXRate="0x00000000"
            modFXDepth="0x00000000"
            delayRate="0x00000000"
            delayFeedback="0x80000000"
            reverbAmount="0x80000000"
            arpeggiatorRate="0x00000000"
            stutterRate="0x00000000"
            sampleRateReduction="0x80000000"
            bitCrush="0x80000000"
            modFXOffset="0x00000000"
            modFXFeedback="0x00000000">
            <envelope1
                attack="0x80000000"
                decay="0xE6666654"
                sustain="0x7FFFFFD2"
                release="0x80000000" />
            <envelope2
                attack="0xE6666654"
                decay="0xE6666654"
                sustain="0xFFFFFFE9"
                release="0xE6666654" />
            <patchCables>
                <patchCable
                    source="velocity"
                    destination="volume"
                    amount="0x3FFFFFE8" />
                <patchCable
                    source="aftertouch"
                    destination="volume"
                    amount="0x2A3D7094" />
                <patchCable
                    source="y"
                    destination="lpfFrequency"
                    amount="0x19999990" />
            </patchCables>
            <equalizer
                bass="0x00000000"
                treble="0x00000000"
                bassFrequency="0x00000000"
                trebleFrequency="0x00000000" />
        </defaultParams>
        <arpeggiator
            mode="off"
            numOctaves="2"
            syncLevel="7" />
        <modKnobs>
            <modKnob controlsParam="pan" />
            <modKnob controlsParam="volumePostFX" />
            <modKnob controlsParam="lpfResonance" />
            <modKnob controlsParam="lpfFrequency" />
            <modKnob controlsParam="env1Release" />
            <modKnob controlsParam="env1Attack" />
            <modKnob controlsParam="delayFeedback" />
            <modKnob controlsParam="delayRate" />
            <modKnob controlsParam="reverbAmount" />
            <modKnob controlsParam="volumePostReverbSend" patchAmountFromSource="compressor" />
            <modKnob controlsParam="pitch" patchAmountFromSource="lfo1" />
            <modKnob controlsParam="lfo1Rate" />
            <modKnob controlsParam="pitch" />
            <modKnob controlsParam="stutterRate" />
            <modKnob controlsParam="bitcrushAmount" />
            <modKnob controlsParam="sampleRateReduction" />
        </modKnobs>
    </sound>
"#;