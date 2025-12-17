import * as api from "./api.js";
import type { Question, Summary } from "./api.js";

export async function revealSummary() {
  // Insert summary into page
  const summaryTemplate = document.querySelector("template#summary-template");
  summaryTemplate || console.error("template#summary-template not found");
  const contentSlot = document.querySelector("#content") || console.error("#content not found");
  contentSlot!.innerHTML = summaryTemplate!.innerHTML;

  await updateSummary();
  setInterval(updateSummary, 60000);
}

function renderSummary(summary: Summary) {
  const summaryContainer = document.querySelector(".summary") || console.error("selector .summary unmatched");

  const questions = {
    "last-24-hours": summary.last_24_hours,
    "last-12-hours": summary.last_12_hours,
    "last-6-hours": summary.last_6_hours,
    "last-3-hours": summary.last_3_hours,
    "last-hour": summary.last_hour,
  };
  const answers = {
    "yes": (q: Question) => q.yes,
    "no": (q: Question) => q.no,
  };
  for (const [questionSelector, question] of Object.entries(questions)) {
    for (const [answerSelector, answer] of Object.entries(answers)) {
      summaryContainer!.querySelector(`.${questionSelector} .${answerSelector}`)!.innerHTML = `${answer(question)}`;
    }
  }
}

async function updateSummary() {
  let summary = await api.summary();
  renderSummary(summary);
}
