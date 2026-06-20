import { gql } from '@apollo/client';

// Mutation to create a new repository
export const CREATE_REPOSITORY = gql`
  mutation CreateRepository($input: CreateRepositoryInput!) {
    createRepository(input: $input) {
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

// Mutation to delete a repository
export const DELETE_REPOSITORY = gql`
  mutation DeleteRepository($id: ID!) {
    deleteRepository(id: $id)
  }
`;

// Mutation to create a new branch
export const CREATE_BRANCH = gql`
  mutation CreateBranch($repositoryId: ID!, $input: CreateBranchInput!) {
    createBranch(repositoryId: $repositoryId, input: $input) {
      id
      repositoryId
      name
      commitHash
      createdAt
    }
  }
`;
