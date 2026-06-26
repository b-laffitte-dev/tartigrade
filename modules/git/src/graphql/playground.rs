//! GraphQL Playground handler for Tardigrade Git
//!
//! This module provides a GraphQL Playground UI for testing GraphQL queries.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use std::sync::Arc;

use crate::AppState;

/// GraphQL Playground HTML template
const PLAYGROUND_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Tardigrade Git - GraphQL Playground</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            margin: 0;
            padding: 0;
            background: #f5f5f5;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            margin-bottom: 20px;
            border-radius: 8px;
        }
        .header h1 {
            margin: 0;
            font-size: 24px;
        }
        .playground-container {
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }
        .tabs {
            display: flex;
            border-bottom: 1px solid #e0e0e0;
        }
        .tab {
            padding: 12px 20px;
            cursor: pointer;
            border: none;
            background: none;
            font-size: 14px;
            color: #666;
            border-bottom: 2px solid transparent;
        }
        .tab.active {
            color: #667eea;
            border-bottom-color: #667eea;
        }
        .tab:hover {
            color: #667eea;
        }
        .editor {
            padding: 20px;
            min-height: 400px;
        }
        .editor textarea {
            width: 100%;
            height: 300px;
            border: 1px solid #e0e0e0;
            border-radius: 4px;
            padding: 10px;
            font-family: 'Monaco', 'Menlo', monospace;
            font-size: 14px;
            resize: none;
        }
        .buttons {
            padding: 10px 20px;
            background: #f8f9fa;
            border-top: 1px solid #e0e0e0;
            display: flex;
            gap: 10px;
        }
        .button {
            padding: 8px 16px;
            background: #667eea;
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
        }
        .button:hover {
            background: #5568d3;
        }
        .button.secondary {
            background: #6c757d;
        }
        .button.secondary:hover {
            background: #5a6268;
        }
        .results {
            padding: 20px;
            border-top: 1px solid #e0e0e0;
            min-height: 200px;
            background: #f8f9fa;
        }
        .results pre {
            margin: 0;
            font-family: 'Monaco', 'Menlo', monospace;
            font-size: 12px;
            white-space: pre-wrap;
            word-wrap: break-word;
        }
        .status {
            padding: 8px 12px;
            background: #d4edda;
            color: #155724;
            border-radius: 4px;
            font-size: 12px;
            margin-bottom: 10px;
            display: inline-block;
        }
        .status.error {
            background: #f8d7da;
            color: #721c24;
        }
        .query-examples {
            margin-top: 20px;
            padding: 15px;
            background: #f8f9fa;
            border-radius: 4px;
        }
        .query-examples h3 {
            margin-top: 0;
            font-size: 14px;
            color: #666;
        }
        .query-example {
            margin-bottom: 10px;
            padding: 8px;
            background: white;
            border-radius: 4px;
            font-family: 'Monaco', 'Menlo', monospace;
            font-size: 12px;
            cursor: pointer;
        }
        .query-example:hover {
            background: #f8f9fa;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🦕 Tardigrade Git - GraphQL Playground</h1>
        </div>
        
        <div class="playground-container">
            <div class="tabs">
                <button class="tab active" onclick="switchTab('query')">Query</button>
                <button class="tab" onclick="switchTab('variables')">Variables</button>
                <button class="tab" onclick="switchTab('headers')">Headers</button>
            </div>
            
            <div id="query-tab" class="editor">
                <textarea id="query-editor" placeholder="Enter your GraphQL query here...">query {
  repositories {
    data {
      id
      name
      description
      isPrivate
      defaultBranch
      createdAt
      updatedAt
    }
    page
    pageSize
    total
    totalPages
  }
}</textarea>
            </div>
            
            <div id="variables-tab" class="editor" style="display: none;">
                <textarea id="variables-editor" placeholder="Enter query variables here...">{
  "page": 1,
  "pageSize": 10
}</textarea>
            </div>
            
            <div id="headers-tab" class="editor" style="display: none;">
                <textarea id="headers-editor" placeholder="Enter request headers here...">{
  "Content-Type": "application/json"
}</textarea>
            </div>
            
            <div class="buttons">
                <button class="button" onclick="executeQuery()">Run Query</button>
                <button class="button secondary" onclick="clearResults()">Clear</button>
            </div>
            
            <div class="results" id="results">
                <span class="status">Ready</span>
                <pre id="results-content"></pre>
            </div>
            
            <div class="query-examples">
                <h3>Query Examples</h3>
                <div class="query-example" onclick="loadExample('repositories')">Get all repositories</div>
                <div class="query-example" onclick="loadExample('repository')">Get a specific repository</div>
                <div class="query-example" onclick="loadExample('branches')">Get branches for a repository</div>
                <div class="query-example" onclick="loadExample('commits')">Get commits for a repository</div>
                <div class="query-example" onclick="loadExample('create_repo')">Create a repository (mutation)</div>
            </div>
        </div>
    </div>
    
    <script>
        function switchTab(tabName) {
            // Hide all tabs
            document.getElementById('query-tab').style.display = 'none';
            document.getElementById('variables-tab').style.display = 'none';
            document.getElementById('headers-tab').style.display = 'none';
            
            // Remove active class from all tab buttons
            document.querySelectorAll('.tab').forEach(tab => {
                tab.classList.remove('active');
            });
            
            // Show selected tab
            if (tabName === 'query') {
                document.getElementById('query-tab').style.display = 'block';
                document.querySelector('.tab:nth-child(1)').classList.add('active');
            } else if (tabName === 'variables') {
                document.getElementById('variables-tab').style.display = 'block';
                document.querySelector('.tab:nth-child(2)').classList.add('active');
            } else if (tabName === 'headers') {
                document.getElementById('headers-tab').style.display = 'block';
                document.querySelector('.tab:nth-child(3)').classList.add('active');
            }
        }
        
        function executeQuery() {
            const query = document.getElementById('query-editor').value;
            const variables = document.getElementById('variables-editor').value;
            const headers = document.getElementById('headers-editor').value;
            
            updateStatus('Loading...', false);
            
            fetch('/api/v1/graphql', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    ...parseHeaders(headers)
                },
                body: JSON.stringify({
                    query: query,
                    variables: parseVariables(variables)
                })
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                return response.json();
            })
            .then(data => {
                displayResults(data);
                updateStatus('Success', true);
            })
            .catch(error => {
                displayResults({ errors: [{ message: error.message }] });
                updateStatus('Error', false);
            });
        }
        
        function parseVariables(variablesStr) {
            try {
                return JSON.parse(variablesStr || '{}');
            } catch (e) {
                return {};
            }
        }
        
        function parseHeaders(headersStr) {
            try {
                const headers = JSON.parse(headersStr || '{}');
                const result = {};
                for (const [key, value] of Object.entries(headers)) {
                    result[key] = value;
                }
                return result;
            } catch (e) {
                return {};
            }
        }
        
        function displayResults(data) {
            const resultsContent = document.getElementById('results-content');
            resultsContent.textContent = JSON.stringify(data, null, 2);
        }
        
        function updateStatus(message, isSuccess) {
            const statusElement = document.querySelector('.status');
            statusElement.textContent = message;
            statusElement.className = isSuccess ? 'status' : 'status error';
        }
        
        function clearResults() {
            document.getElementById('results-content').textContent = '';
            updateStatus('Ready', true);
        }
        
        function loadExample(exampleName) {
            const examples = {
                'repositories': `query {
  repositories {
    data {
      id
      name
      description
      isPrivate
      defaultBranch
      createdAt
      updatedAt
    }
    page
    pageSize
    total
    totalPages
  }
}`,
                'repository': `query {
  repository(id: "REPLACE_WITH_ID") {
    id
    name
    description
    isPrivate
    defaultBranch
    createdAt
    updatedAt
  }
}`,
                'branches': `query {
  branches(repositoryId: "REPLACE_WITH_ID") {
    data {
      id
      repositoryId
      name
      commitHash
      isDefault
      createdAt
      updatedAt
    }
    page
    pageSize
    total
    totalPages
  }
}`,
                'commits': `query {
  commits(repositoryId: "REPLACE_WITH_ID") {
    data {
      id
      repositoryId
      hash
      parentHash
      message
      authorName
      authorEmail
      committerName
      committerEmail
      branchName
      createdAt
    }
    page
    pageSize
    total
    totalPages
  }
}`,
                'create_repo': `mutation {
  createRepository(input: {
    name: "my-new-repo",
    description: "A test repository",
    isPrivate: false,
    defaultBranch: "main"
  }) {
    id
    name
    description
    isPrivate
    defaultBranch
    createdAt
    updatedAt
  }
}`
            };
            
            document.getElementById('query-editor').value = examples[exampleName] || '';
            switchTab('query');
        }
    </script>
</body>
</html>
"#;

/// GraphQL Playground handler
pub async fn graphql_playground_handler() -> impl IntoResponse {
    Html(PLAYGROUND_HTML)
}

/// GraphQL handler for POST requests
pub async fn graphql_handler(
    State(state): State<Arc<AppState>>,
    request: axum::extract::Json<async_graphql::Request>,
) -> Result<axum::Json<async_graphql::Response>, StatusCode> {
    use super::schema::GitSchema;
    
    let schema = GitSchema::new();
    
    // Execute the GraphQL query
    let response = schema
        .execute(
            request.0.query,
            request.0.operation_name,
            request.0.variables,
            &state,
        )
        .await;
    
    Ok(axum::Json(response))
}

/// Combined GraphQL handler that serves both GET (playground) and POST (queries)
pub async fn graphql_combined_handler(
    State(state): State<Arc<AppState>>,
    request: axum::http::Request<axum::body::Body>,
) -> Result<impl IntoResponse, StatusCode> {
    if request.method() == axum::http::Method::GET {
        Ok(graphql_playground_handler().await)
    } else {
        let (parts, body) = request.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX).await?;
        let request: async_graphql::Request = serde_json::from_slice(&bytes)
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        
        graphql_handler(State(state), axum::extract::Json(request)).await
    }
}
