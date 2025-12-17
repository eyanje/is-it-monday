const TIMEOUT = 3600_000;

export class Submission {
  submittedAt: Date;

  constructor(submittedAt: Date) {
    this.submittedAt = submittedAt;
  }

  // Retrieve the last submission.
  static last(): (Submission | null) {
    const submittedAt = localStorage.getItem("last-submitted");

    return submittedAt && new Submission(new Date(submittedAt)) || null;
  }

  // True if the submission has a valid date within an hour.
  isValid(now: Date): boolean {
    return !isNaN(this.submittedAt.valueOf()) &&
      (now.valueOf() - this.submittedAt.valueOf()) < TIMEOUT;
  }

  // Save the submission to local storage.
  save() {
    localStorage.setItem("last-submitted", this.submittedAt.toISOString());
  }
}

export function updateLastSubmitted(submission: Submission) {
  const lastSubmittedField = document.querySelector(".last-submitted");
  if (lastSubmittedField === null) {
    throw new Error(".last-submitted not found");
  }

  const submittedAt = submission.submittedAt;
  const openAt = new Date(submission.submittedAt.valueOf() + TIMEOUT);
  const now = new Date();

  lastSubmittedField.innerHTML = `Last submitted at ${submittedAt.toLocaleString()}.`;
  if (submission.isValid(now)) {
    lastSubmittedField.innerHTML += ` No resubmissions until ${openAt.toLocaleTimeString()}.`;
  }
}

