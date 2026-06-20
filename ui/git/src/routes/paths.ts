export const PATHS = {
  home: '/',
  repository: {
    list: '/repositories',
    create: '/repositories/create',
    detail: (id: string) => `/repositories/${id}`,
    branches: (id: string) => `/repositories/${id}/branches`,
    branchesCreate: (id: string) => `/repositories/${id}/branches/create`,
  },
};
