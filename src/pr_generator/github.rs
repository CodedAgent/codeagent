use super::generator::PRMetadata;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubPRCreateRequest {
    pub title: String,
    pub body: String,
    pub head: String,
    pub base: String,
    pub draft: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubPRResponse {
    pub id: u64,
    pub number: u32,
    pub title: String,
    pub html_url: String,
    pub state: String,
}

pub struct GitHubIntegration {
    token: String,
    owner: String,
    repo: String,
    base_url: String,
    client: reqwest::Client,
}

impl GitHubIntegration {
    pub fn new(token: String, owner: String, repo: String) -> Self {
        GitHubIntegration {
            token,
            owner,
            repo,
            base_url: "https://api.github.com".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn create_pr(
        &self,
        meta: &PRMetadata,
        description: &str,
        draft: bool,
    ) -> Result<GitHubPRResponse> {
        let url = format!(
            "{}/repos/{}/{}/pulls",
            self.base_url, self.owner, self.repo
        );

        let request = GitHubPRCreateRequest {
            title: meta.title.clone(),
            body: description.to_string(),
            head: meta.target_branch.clone(),
            base: meta.base_branch.clone(),
            draft,
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let pr: GitHubPRResponse = response.json().await?;
            tracing::info!("Created PR #{}: {}", pr.number, pr.title);
            Ok(pr)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("GitHub API error: {}", error_text))
        }
    }

    pub async fn add_reviewers(&self, pr_number: u32, reviewers: &[String]) -> Result<()> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}/requested_reviewers",
            self.base_url, self.owner, self.repo, pr_number
        );

        #[derive(Serialize)]
        struct ReviewRequest {
            reviewers: Vec<String>,
        }

        let request = ReviewRequest {
            reviewers: reviewers.to_vec(),
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            tracing::info!("Added {} reviewers to PR #{}", reviewers.len(), pr_number);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to add reviewers"))
        }
    }

    pub async fn add_labels(&self, pr_number: u32, labels: &[String]) -> Result<()> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/labels",
            self.base_url, self.owner, self.repo, pr_number
        );

        #[derive(Serialize)]
        struct LabelRequest {
            labels: Vec<String>,
        }

        let request = LabelRequest {
            labels: labels.to_vec(),
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            tracing::info!("Added {} labels to PR #{}", labels.len(), pr_number);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to add labels"))
        }
    }

    pub async fn close_pr(&self, pr_number: u32) -> Result<()> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}",
            self.base_url, self.owner, self.repo, pr_number
        );

        #[derive(Serialize)]
        struct CloseRequest {
            state: String,
        }

        let request = CloseRequest {
            state: "closed".to_string(),
        };

        let response = self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            tracing::info!("Closed PR #{}", pr_number);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to close PR"))
        }
    }
}
