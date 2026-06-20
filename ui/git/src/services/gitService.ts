import { ApolloClient, InMemoryCache, HttpLink, setContext } from '@apollo/client';
import { onError } from '@apollo/client/link/error';
import { RetryLink } from '@apollo/client/link/retry';
import toast from 'react-hot-toast';

const GRAPHQL_ENDPOINT = import.meta.env.VITE_GRAPHQL_ENDPOINT || 'http://localhost:3001/graphql';

export const createApolloClient = (token?: string) => {
  const httpLink = new HttpLink({ uri: GRAPHQL_ENDPOINT });

  const authLink = setContext((_, { headers }) => ({
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : '',
    },
  }));

  const errorLink = onError(({ graphQLErrors, networkError }) => {
    graphQLErrors?.forEach(({ message }) => {
      console.error(`[GraphQL error]: ${message}`);
      toast.error(`Erreur: ${message}`);
    });
    if (networkError) {
      console.error(`[Network error]: ${networkError}`);
      toast.error('Erreur réseau');
    }
  });

  const retryLink = new RetryLink({
    delay: { initial: 300, max: Infinity, jitter: true },
    attempts: { max: 5, retryIf: (e) => !!e },
  });

  return new ApolloClient({
    link: errorLink.concat(retryLink.concat(authLink.concat(httpLink))),
    cache: new InMemoryCache(),
  });
};

let apolloClient: ApolloClient<any> | null = null;

export const getApolloClient = (token?: string) => {
  if (!apolloClient) apolloClient = createApolloClient(token);
  return apolloClient;
};

export const resetApolloClient = () => {
  apolloClient = null;
};
