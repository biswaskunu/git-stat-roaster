import { useState } from "react";

function fmtRepo(repo, unitLabel) {
  if (!repo) return "—";
  return `${repo.name} (${repo[unitLabel]})`;
}

function statRows(data) {
  return [
    ["account age", `${data.account_age_years}y`],
    ["repos", `${data.public_repos} (${data.non_fork_repo_count} non-fork)`],
    ["followers", `${data.followers}`],
    ["following", `${data.following}`],
    ["bio", data.bio || "—"],
    ["top language", data.most_used_language || "—"],
    ["most starred", fmtRepo(data.top_starred_repo, "stars")],
    ["most forked", fmtRepo(data.top_forked_repo, "forks")],
    ["oldest repo", data.oldest_repo ? data.oldest_repo.name : "—"],
  ];
}

export default function App() {
  const [username, setUsername] = useState("");
  const [data, setData] = useState(null);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

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
        setError(json.error || "Something went wrong.");
        return;
      }

      setData(json);
    } catch (err) {
      setError("Couldn't reach the server. Is the backend running?");
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="page">
      <div className="ticket">
        <header className="ticket-head">
          <h1>git-stat-roaster</h1>
          <p className="tagline">Hand over a GitHub username. Walk away with a receipt.</p>
        </header>

        <form onSubmit={handleSubmit} className="lookup-form">
          <label htmlFor="username-input" className="prompt">$</label>
          <input
            id="username-input"
            type="text"
            placeholder="github-username"
            autoComplete="off"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
          />
          <button type="submit" disabled={loading}>
            {loading ? "pulling stats…" : "roast"}
          </button>
        </form>

        {error && <p className="error-line">✕ {error}</p>}

        {data && (
          <section className="result">
            <div className="verdict">
              <span className="verdict-label">the verdict</span>
              <p className="verdict-text">{data.roast}</p>
            </div>

            <dl className="ledger">
              {statRows(data).map(([label, value]) => (
                <div className="ledger-row" key={label}>
                  <dt>{label}</dt>
                  <span className="leader" aria-hidden="true" />
                  <dd>{value}</dd>
                </div>
              ))}
            </dl>

            <p className="ticket-footer">— issued for @{data.username} —</p>
          </section>
        )}
      </div>
    </main>
  );
}