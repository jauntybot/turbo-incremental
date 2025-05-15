use super::*;
use once_cell::sync::Lazy;

pub static CUTSCENES: Lazy<Vec<Dialogue>> = Lazy::new(|| vec![
    Dialogue {
        messages: vec![
            "Exoplanet detected!".to_string(),
            "Sending autonomous research probe to Exoplanet...".to_string(),
            "Scan the Exoplanet to gather scientific RESEARCH and report back.  ".to_string(),
        ],
        camera_pos: vec![((320, 296), 0)],
        d_box: DialogueBox::new(),
        event_broadcast: 0,
    },
    Dialogue {
        messages: vec![
            "Significant RESEARCH gathered from research probe!".to_string(),
            "Authorizing construction of DRONE DEPOT. ".to_string(),
            "Establish a hub for additional autonomous workers and deploy them to gather RESEARCH.".to_string(),
        ],
        camera_pos: vec![((320, 200), 0), ((DEPOT_BOX.0 + DEPOT_BOX.2/2, DEPOT_BOX.1 - 16), 1)],
        d_box: DialogueBox::new(),
        event_broadcast: 1,
    },
    Dialogue {
        messages: vec![
            "Automated RESEARCH production initiated.".to_string(),
            "New scans revealed nearby mineral rich asteroid belt!".to_string(),
            "Authorizing construction of ASTEROID MINES. ".to_string(),
            "Gather METALS from the asteroids to build advanced tech.".to_string(),
        ],
        camera_pos: vec![((320, 200), 0), ((MINES_BOX.0 - 16, MINES_BOX.1 + MINES_BOX.3/2), 2)],
        d_box: DialogueBox::new(),
        event_broadcast: 2,
    },
    Dialogue {
        messages: vec![
            "Automated METALS production initiated.".to_string(),
            "Further scans have revealed nearby nebula storm.".to_string(),
            "Authorizing construction of POWER PLANT.".to_string(),
            "Harvest POWER from the storm to amplify other stations.".to_string(),
        ],
        camera_pos: vec![((64, 32), 0), ((PLANT_BOX.0 + PLANT_BOX.2/2, PLANT_BOX.1 - 16), 2)],
        d_box: DialogueBox::new(),
        event_broadcast: 2,
    },
    Dialogue {
        messages: vec![
            "Automated POWER production initiated.".to_string(),
            "Sector self-sufficiency achieved. Final stage of .".to_string(),
            // "Authorizing construction of RESEARCH COMPLEX.".to_string(),
            // ".".to_string(),
            "Authorizing construction of JUMPGATE.".to_string(),
            "Use the JUMPGATE to leave this sector and start in a new sector.".to_string(),
        ],
        camera_pos: vec![((PLANT_BOX.0 + PLANT_BOX.2/2, PLANT_BOX.1 + PLANT_BOX.3/2), 0), ((GATE_BOX.0 + GATE_BOX.2/2, GATE_BOX.1 - 16), 2)], // ((COMPLEX_BOX.0 + COMPLEX_BOX.2/2, COMPLEX_BOX.1 + COMPLEX_BOX.3/2), 2),
        d_box: DialogueBox::new(),
        event_broadcast: 2,
    },
        Dialogue {
        messages: vec![
            "Jumpgate initiated. Prepare for emminent jump.".to_string(),
            "Sector self-sufficiency achieved. Final stage of .".to_string(),
        ],
        camera_pos: vec![((GATE_BOX.0 + GATE_BOX.2/2, GATE_BOX.1 - 16), 0)], // ((COMPLEX_BOX.0 + COMPLEX_BOX.2/2, COMPLEX_BOX.1 + COMPLEX_BOX.3/2), 2),
        d_box: DialogueBox::new(),
        event_broadcast: 1,
    },
]);