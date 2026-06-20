import { gql } from '@apollo/client';

// Query to list repositories with pagination
export const LIST_REPOSITORIES = gql`
  query ListRepositories($ownerId: ID, $page: Int = 1, $pageSize: Int = 20) {
    repositories(ownerId: $ownerId, page: $page, pageSize: $pageSize) {
      data {
        id
        name
        description
        isPrivate
        ownerId
        defaultBranch
        createdAt
        updatedAt
      }
      page
      pageSize
      total
      totalPages
    }
  }
`;

// Query to get a single repository by ID
export const GET_REPOSITORY = gql`
  query GetRepository($id: ID!) {
    repository(id: $id) {
      id
      name
      description
      isPrivate
      ownerId
      defaultBranch
      createdAt
      updatedAt
    }
  }
`;

// Query to list branches for a repository
export const LIST_BRANCHES = gql`
  query ListBranches($repositoryId: ID!, $page: Int = 1, $pageSize: Int = 20) {
    branches(repositoryId: $repositoryId, page: $page, pageSize: $pageSize) {
      data {
        id
        repositoryId
        name
        commitHash
        createdAt
      }
      page
      pageSize
      total
      totalPages
    }
  }
`;
