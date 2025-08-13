use super::*;
use once_cell::sync::Lazy;

pub static CS_INTRO: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "EXOPLANET detected!".to_string(),
        "Sending autonomous RESEARCH PROBE to EXOPLANET...".to_string(),
        "Scan the EXOPLANET to gather scientific RESEARCH.  ".to_string(),
    ],
    camera_pos: vec![((320, 200), 0), ((320, 256), 2)],
    d_box: DialogueBox::new(),
    event_broadcast: 2,
    prompt: false,
});

pub static CS_DEPOT_AVAIL: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Significant RESEARCH gathered by RESEARCH PROBE!".to_string(),
        "Authorizing construction of DRONE DEPOT. ".to_string(),
        "Establish a hub for additional autonomous workers and deploy them to gather RESEARCH.".to_string(),
    ],
    camera_pos: vec![((320, 200), 0), ((DEPOT_BOX.0 + DEPOT_BOX.2/2, DEPOT_BOX.1 - 16), 1)],
    d_box: DialogueBox::new(),
    event_broadcast: 1,
    prompt: false,
});

pub static CS_MINES_AVAIL: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Automated RESEARCH production initiated.".to_string(),
        "Hover over the DRONE count in the top right to inspect their production!".to_string(),
        "New scans revealed nearby mineral rich asteroid belt!".to_string(),
        "Authorizing construction of ASTEROID MINES. ".to_string(),
        "Gather METALS from the asteroids to build advanced tech.".to_string(),
    ],
    camera_pos: vec![((320, 200), 0), ((MINES_BOX.0 - 16, MINES_BOX.1 + MINES_BOX.3/2), 2)],
    d_box: DialogueBox::new(),
    event_broadcast: 2,
    prompt: false,
});

pub static CS_PLANT_AVAIL: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Automated METALS production initiated.".to_string(),
        "Further scans have revealed nearby nebula storm.".to_string(),
        "Authorizing construction of POWER PLANT.".to_string(),
        "Harvest POWER from the storm to amplify other stations.".to_string(),
    ],
    camera_pos: vec![((64, 32), 0), ((PLANT_BOX.0 + PLANT_BOX.2/2, PLANT_BOX.1 - 16), 2)],
    d_box: DialogueBox::new(),
    event_broadcast: 2,
    prompt: false,
});

pub static CS_LATE_GAME: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Automated POWER production initiated.".to_string(),
        "Sector self-sufficiency achieved. Entering final stage of exoplanet observation.".to_string(),
        "Authorizing construction of RESEARCH COMPLEX.".to_string(),
        "Assign drones to research new projects to complete in this sector.".to_string(),
        "Authorizing construction of JUMPGATE.".to_string(),
        "If progress is slowing down, use the JUMPGATE to PRESTIGE.".to_string(),
    ],
    camera_pos: vec![((PLANT_BOX.0 + PLANT_BOX.2/2, PLANT_BOX.1 + PLANT_BOX.3/2), 0), ((COMPLEX_BOX.0 + COMPLEX_BOX.2/2, COMPLEX_BOX.1 + COMPLEX_BOX.3/2), 2), ((GATE_BOX.0 + GATE_BOX.2/2, GATE_BOX.1 - 32), 4)], 
    d_box: DialogueBox::new(),
    event_broadcast: 2,
    prompt: false,
});

pub static CS_JUMPGATE_BUILT: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "JUMPGATE constructed! Use the JUMPGATE to restart in a new sector when progress slows.".to_string(),
        "The JUMPGATE tracks ALL RESOURCES gathered and rewards PRESTIGE when used.".to_string(),
        "After jumping, hover over the RESEARCH PROBE to upgrade it with PRESTIGE.".to_string(),
    ],
    camera_pos: vec![((GATE_BOX.0 + GATE_BOX.2/2, GATE_BOX.1 - 32), 0)], 
    d_box: DialogueBox::new(),
    event_broadcast: 2,
    prompt: false,
});

pub static CS_JUMP: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Jumpgate initiated. Prepare for imminent jump.".to_string(),
        "Good work, researcher! There's more work in the next sector.".to_string(),
    ],
    camera_pos: vec![((GATE_BOX.0 + GATE_BOX.2/2, GATE_BOX.1 - 32), 0)], 
    d_box: DialogueBox::new(),
    event_broadcast: 0,
    prompt: false,
});

pub static CS_COMPLEX_BUILT: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "RESEARCH COMPLEX constructed! Assign DRONES to gather more RESEARCH.".to_string(),
        "RESEARCH gathered here is collected AND used towards RESEARCH PROJECTS.".to_string(),
        "Assign a RESEARCH PROJECT to start contributing to it.".to_string(),
    ],
    camera_pos: vec![((COMPLEX_BOX.0 + COMPLEX_BOX.2/2 - 36, COMPLEX_BOX.1 + COMPLEX_BOX.3/2), 0)], 
    d_box: DialogueBox::new(),
    event_broadcast: 2,
    prompt: false,
});

pub static CS_RESET_PROMPT: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Reset all your progress including PRESTIGE?".to_string(),
    ],
    camera_pos: vec![((320, 240), 0)], 
    d_box: DialogueBox::new(),
    event_broadcast: 1,
    prompt: true,
});

pub static CS_PRESTIGE_PROMPT: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Earn PRESTIGE and start again in a new sector?".to_string(),
    ],
    camera_pos: vec![((320, 240), 0)], 
    d_box: DialogueBox::new(),
    event_broadcast: 1,
    prompt: true,
});

pub static CS_OUTRO: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Another sector is waiting observation!".to_string(),
        "Use earned PRESTIGE to upgrade the RESEARCH PROBE!".to_string(),
        "".to_string(),
    ],
    camera_pos: vec![((320, 200), 0)], 
    d_box: DialogueBox::new(),
    event_broadcast: 2,
    prompt: false,
});


pub static CS_FAB_AVAIL: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Research project complete!".to_string(),
        "Authorizing construction of FABRICATOR.".to_string(),
        "Assign drones to siphon METALS in order to fabricate more DRONES!".to_string(),
    ],
    camera_pos: vec![((COMPLEX_BOX.0 + COMPLEX_BOX.2/2, COMPLEX_BOX.1 + COMPLEX_BOX.3/2), 0), ((DEPOT_BOX.0 + DEPOT_BOX.2/2, DEPOT_BOX.1 + DEPOT_BOX.3 - 16), 1)],
    d_box: DialogueBox::new(),
    event_broadcast: 1,
    prompt: false,
});

pub static CS_AMP_AVAIL: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Research project complete!".to_string(),
        "Authorizing construction of DRONE AMP.".to_string(),
        "Activate this station to siphon POWER and supercharge other stations!".to_string(),
    ],
    camera_pos: vec![((COMPLEX_BOX.0 + COMPLEX_BOX.2/2, COMPLEX_BOX.1 + COMPLEX_BOX.3/2), 0), ((AMP_BOX.0 + AMP_BOX.2/2, AMP_BOX.1 + AMP_BOX.3/2), 1)],
    d_box: DialogueBox::new(),
    event_broadcast: 1,
    prompt: false,
});

pub static CS_ADV_DRONES: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Research project complete!".to_string(),
        "All DRONES now have double EFFICENCY!".to_string(),
    ],
    camera_pos: vec![((COMPLEX_BOX.0 + COMPLEX_BOX.2/2 - 36, COMPLEX_BOX.1 + COMPLEX_BOX.3/2), 0)],
    d_box: DialogueBox::new(),
    event_broadcast: 0,
    prompt: false,
});

pub static CS_SIMULACRUM: Lazy<Dialogue> = Lazy::new(|| Dialogue {
    messages: vec![
        "Research project complete!".to_string(),
        //"Authorizing construction of SIMULACRUM.".to_string(),
        //"Constructing the SIMULACRUM will end your observation expedition.".to_string(),
        "The SIMULACRUM is a perfect simulation of the EXOPLANET.".to_string(),
        "Now we can observe the SIMULACRUM remotely from our home system.".to_string(),
        "Congratulations, researcher! You have reached the end of this expedition!".to_string(),
        "There will be more to come in future updates. But for now, play again?".to_string(),
    ],
    camera_pos: vec![((COMPLEX_BOX.0 + COMPLEX_BOX.2/2 - 36, COMPLEX_BOX.1 + COMPLEX_BOX.3/2), 0)],
    d_box: DialogueBox::new(),
    event_broadcast: 4,
    prompt: false,
});