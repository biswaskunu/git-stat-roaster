const form = document.getElementById("roast-form");
const input = document.getElementById("username-input");
const statusEl = document.getElementById("status");
const resultEl = document.getElementById("result");
const roastLineEl = document.getElementById("roast-line");
const statsGridEl = document.getElementById("stats-grid");
const submitButton = form.querySelector("button");

function fmtRepo(repo, unitLabel) {
  if (!repo) return "—";
  return `${repo.name} (${repo[unitLabel]})`;
}

function renderStats(data) {
  const rows = [
    ["Account age", `${data.account_age_years} years`],
    ["Public repos", `${data.public_repos} (${data.non_fork_repo_count} non-fork)`],
    ["Followers / following", `${data.followers} / ${data.following}`],
    ["Bio", data.bio || "—"],
    ["Most-used language", data.most_used_language || "—"],
    ["Most-starred repo", fmtRepo(data.top_starred_repo, "stars")],
    ["Most-forked repo", fmtRepo(data.top_forked_repo, "forks")],
    ["Oldest repo", data.oldest_repo ? data.oldest_repo.name : "—"],
  ];

  statsGridEl.innerHTML = rows
    .map(([label, value]) => `<dt>${label}</dt><dd>${value}</dd>`)
    .join("");
}

form.addEventListener("submit", async (e) => {
  e.preventDefault();
  const username = input.value.trim();
  if (!username) return;

  statusEl.textContent = "";
  resultEl.classList.add("hidden");
  submitButton.disabled = true;
  submitButton.textContent = "Roasting…";

  try {
    const res = await fetch(`/api/roast/${encodeURIComponent(username)}`);
    const data = await res.json();

    if (!res.ok) {
      statusEl.textContent = data.error || "Something went wrong.";
      return;
    }

    roastLineEl.textContent = data.roast;
    renderStats(data);
    resultEl.classList.remove("hidden");
  } catch (err) {
    statusEl.textContent = "Couldn't reach the server. Is the backend running?";
  } finally {
    submitButton.disabled = false;
    submitButton.textContent = "Roast me";
  }
});
