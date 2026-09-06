import { useState } from "react";

function fmtRepo(repo, unitLabel) {
  if (!repo) return "—";
  return `${repo.name} (${repo[unitLabel]})`;
}

function statRows(data) {
  return [
    ["Account age", `${data.account_age_years} years`],
    ["Public repos", `${data.public_repos} (${data.non_fork_repo_count} non-fork)`],
    ["Followers / following", `${data.followers} / ${data.following}`],
    ["Bio", data.bio || "—"],
    ["Most-used language", data.most_used_language || "—"],
    ["Most-starred repo", fmtRepo(data.top_starred_repo, "stars")],
    ["Most-forked repo", fmtRepo(data.top_forked_repo, "forks")],
    ["Oldest repo", data.oldest_repo ? data.oldest_repo.name : "—"],
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
    <main className="wrap">
      <h1>git-stat-roaster</h1>
      <p className="tagline">
        Type a GitHub username. Get judged by your own public stats.
      </p>

      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="e.g. biswaskunu"
          autoComplete="off"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          required
        />
        <button type="submit" disabled={loading}>
          {loading ? "Roasting…" : "Roast me"}
        </button>
      </form>

      {error && <div className="status">{error}</div>}

      {data && (
        <section className="result">
          <p className="roast-line">{data.roast}</p>
          <dl className="stats-grid">
            {statRows(data).map(([label, value]) => (
              <div className="stat-row" key={label}>
                <dt>{label}</dt>
                <dd>{value}</dd>
              </div>
            ))}
          </dl>
        </section>
      )}
    </main>
  );
}
