import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { MainLayout } from '../components/layout/MainLayout';
import { Dashboard } from '../pages/Dashboard';
import { RepositoryList } from '../pages/Repository/List';
import { RepositoryCreate } from '../pages/Repository/Create';
import { RepositoryDetail } from '../pages/Repository/Detail';
import { BranchList } from '../pages/Branches/List';
import { PATHS } from './paths';

const router = createBrowserRouter([
  {
    path: PATHS.home,
    element: <MainLayout />,
    children: [
      { index: true, element: <Dashboard /> },
      { path: PATHS.repository.list, element: <RepositoryList /> },
      { path: PATHS.repository.create, element: <RepositoryCreate /> },
      {
        path: PATHS.repository.detail(':id'),
        element: <RepositoryDetail />,
        children: [
          { index: true, element: <RepositoryDetail /> },
          { path: PATHS.repository.branches(':repositoryId'), element: <BranchList /> },
        ],
      },
    ],
  },
  { path: '*', element: <div>404 - Page non trouvée</div> },
]);

export const AppRouter = () => <RouterProvider router={router} />;
