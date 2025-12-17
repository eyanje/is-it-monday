const endpoint = 'http://eyanje-debian:3000';

export interface Question {
  yes: number,
  no: number,
}

export interface Summary {
  last_24_hours: Question,
  last_12_hours: Question,
  last_6_hours: Question,
  last_3_hours: Question,
  last_hour: Question,
}

export async function submit(answer: boolean) {
  let response = await fetch(endpoint, {
    method: 'POST', 
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(answer)
  });
  if (!await response.ok) {
    throw new Error(await response.text());
  }
}

export async function summary() {
  let response = await fetch(endpoint);
  if (!await response.ok) {
    console.error("Summary", response);
  }
  return await response.json() as Summary;
}
