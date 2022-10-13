mod challenge;

use std::time::Instant;

/// e.g.
/// {
///   "challenge": <Challenge>,
///   "id": "SOME-UUID-FROM-SERVER",
/// }
#[derive(Debug, serde::Deserialize)]
struct ChallengeRequest {
    /* FILL ME */
}

/// e.g.
/// {
///   "responder_name": "Ryan Reynolds",
///   "id": "THAT-UUID-WE-GOT",
///   "answer": "the-answer",
/// }
#[derive(Debug, serde::Serialize)]
struct Solution {
    /* FILL ME */
}

const HOST: &'static str = "https://server-sortinghunt.shuttleapp.rs";

const CONTESTANT_NAME: &'static str = FILL_ME;

fn main() -> Result<(), anyhow::Error> {
    let client = reqwest::blocking::Client::new();

    let challenge_uri = format!("{HOST}/starship/challenge");
    let resp = client.get(challenge_uri).send()?.json::<ChallengeRequest>()?;
    println!("Got challenge ID {}, length {} entries", resp.id, resp.challenge.numbers.len());

    let mut t = Instant::now();
    let sorted = resp.challenge.sorted();
    println!("Sorted in {} us", (Instant::now() - t).as_micros());

    t = Instant::now();
    let answer = sorted.answer().unwrap();
    println!("Calculated answer in {} us", (Instant::now() - t).as_micros());

    println!("Calculated answer: {answer:?}");

    let solution = Solution {
        responder_name: CONTESTANT_NAME.to_string(),
        id: resp.id,
        answer,
    };

    let submit_uri = format!("{HOST}/starship/submit");
    let resp = client
        .post(submit_uri)
        .json(&solution)
        .send()?;
    println!("Server responded: {}", resp.text()?);

    Ok(())
}
