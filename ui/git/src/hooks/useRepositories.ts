import { useQuery, useMutation, useQueryClient } from '@apollo/client';
import { LIST_REPOSITORIES, GET_REPOSITORY, CREATE_REPOSITORY, DELETE_REPOSITORY } from '../graphql/queries';
import { CreateRepositoryInput, PaginatedResponse, Repository } from '../types/git';

export const useRepositories = (ownerId?: string, page: number = 1, pageSize: number = 20) => {
  const { data, loading, error, refetch } = useQuery(LIST_REPOSITORIES, {
    variables: { ownerId, page, pageSize },
    fetchPolicy: 'cache-and-network',
  });

  return {
    repositories: data?.repositories as PaginatedResponse<Repository> | undefined,
    loading,
    error,
    refetch,
  };
};

export const useRepository = (id: string) => {
  const { data, loading, error, refetch } = useQuery(GET_REPOSITORY, {
    variables: { id },
    skip: !id,
  });

  return {
    repository: data?.repository as Repository | undefined,
    loading,
    error,
    refetch,
  };
};

export const useCreateRepository = () => {
  const queryClient = useQueryClient();
  const [createRepository, { loading, error }] = useMutation(CREATE_REPOSITORY, {
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['repositories'] });
    },
  });

  return { createRepository, loading, error };
};

export const useDeleteRepository = () => {
  const queryClient = useQueryClient();
  const [deleteRepository, { loading, error }] = useMutation(DELETE_REPOSITORY, {
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['repositories'] });
      queryClient.invalidateQueries({ queryKey: ['repository', id] });
    },
  });

  return { deleteRepository, loading, error };
};
