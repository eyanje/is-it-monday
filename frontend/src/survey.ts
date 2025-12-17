import * as api from "./api.js";
import { Submission, updateLastSubmitted } from "./submission.js";
import { revealSummary } from "./summary.js";

async function submitSurvey(event: SubmitEvent) {
  event.preventDefault();

  const form: HTMLFormElement = event.target! as HTMLFormElement;
  const formData = new FormData(form);

  // Submit form response
  let formResponse;
  switch (formData.get("today")) {
    case "monday":
      formResponse = true;
    break;
    case "not-monday":
      formResponse = false;
    break;
    default:
      throw new Error("missing response");
  }
  await api.submit(formResponse);

  // Log history
  const submission = new Submission(new Date())
  submission.save();

  await revealContent();
}

export function revealSurvey() {
  const formTemplate: HTMLTemplateElement | null = document.querySelector("#form-template");
  formTemplate || console.error("#form-template not found");
  const content = document.querySelector("#content") || console.error("#content not found");
  content!.innerHTML = formTemplate!.innerHTML;

  const form: HTMLFormElement | null = document.querySelector("form.survey");
  form || console.error("Selected element form.survey not found");
  form?.addEventListener("submit", submitSurvey);
}

export async function revealContent() {
  // Check last recorded submission
  const now = new Date();
  const lastSubmission = Submission.last();
  lastSubmission && updateLastSubmitted(lastSubmission);

  if (lastSubmission !== null && !lastSubmission.isValid(now)) {
    revealSurvey();
  } else {
    await revealSummary();
  }
}

revealContent();
