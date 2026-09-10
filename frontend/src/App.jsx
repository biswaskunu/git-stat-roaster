import { useState } from "react";

const LOADING_LINES = [
  "cloning your dignity…",
  "resolving merge conflicts with your ego…",
  "running git blame on your life choices…",
  "force-pushing to main…",
];

function fmtRepo(repo, unitLabel) {
  if (!repo) return "—";
  return `${repo.name} (${repo[unitLabel]})`;
}

function statRows(data) {
  return [
    ["age", `${data.account_age_years}y on the platform`],
    ["repos", `${data.public_repos} total · ${data.non_fork_repo_count} non-fork`],
    ["followers", `${data.followers} follower${data.followers === 1 ? "" : "s"} / ${data.following} following`],
    ["bio", data.bio || "empty string"],
    ["language", data.most_used_language || "undefined"],
    ["top star", fmtRepo(data.top_starred_repo, "stars")],
    ["top fork", fmtRepo(data.top_forked_repo, "forks")],
    ["oldest", data.oldest_repo ? data.oldest_repo.name : "—"],
  ];
}

export default function App() {
  const [username, setUsername] = useState("");
  const [data, setData] = useState(null);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [loadingLine] = useState(
    () => LOADING_LINES[Math.floor(Math.random() * LOADING_LINES.length)]
  );

  async function handleSubmit(e) {
    e.preventDefault();
    const trimmed = username.trim();
    if (!trimmed) return;

    setError("");
    setData(null);
    setLoading(true);

    try {
      const res = await fetch(`/api/roast/${encodeURIComponent(trimmed)}`);
      const json = await res.json();

      if (!res.ok) {
        setError(json.error || "something went wrong. blame github, not us.");
        return;
      }

      setData(json);
    } catch (err) {
      setError("couldn't reach the server. is it running? did you check?");
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="wrap">
      <header className="masthead">
        <div className="prompt">
          <span className="prompt-sigil">$</span> git-stat-roaster
          <span className="cursor" aria-hidden="true" />
        </div>
        <p className="tagline">
          Point it at a GitHub username. It reads your public commit history
          and judges you accordingly.
        </p>
      </header>

      <form onSubmit={handleSubmit} className="lookup">
        <label className="field">
          <span className="field-label">username</span>
          <input
            type="text"
            placeholder="octocat"
            autoComplete="off"
            spellCheck="false"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
          />
        </label>
        <button type="submit" disabled={loading}>
          {loading ? "roasting…" : "roast me"}
        </button>
      </form>

      {loading && <p className="status status-loading">{loadingLine}</p>}
      {error && <p className="status status-error">{error}</p>}

      {data && (
        <section className="result">
          <blockquote className="roast-line">
            <span className="quote-mark" aria-hidden="true">
              “
            </span>
            {data.roast}
          </blockquote>

          <dl className="diff">
            {statRows(data).map(([label, value]) => (
              <div className="diff-row" key={label}>
                <dt>{label}</dt>
                <dd>{value}</dd>
              </div>
            ))}
          </dl>

          <p className="footnote">
            no commits were harmed in the making of this roast.
          </p>
        </section>
      )}
    </main>
  );
}
