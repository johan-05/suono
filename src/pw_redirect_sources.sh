pw-link -d "alsa_input.pci-0000_03_00.6.HiFi__Mic1__source:capture_FR" "alsa_capture.suono:input_FR"
pw-link -d "alsa_input.pci-0000_03_00.6.HiFi__Mic1__source:capture_FL" "alsa_capture.suono:input_FL"

pw-link "spotify:output_FL" "alsa_capture.suono:input_FL"
pw-link "spotify:output_FR" "alsa_capture.suono:input_FR"
