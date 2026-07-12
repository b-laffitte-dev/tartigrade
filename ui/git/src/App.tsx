// =============================================================================
// Tardigrade-CI Git Module - Main App Component
// =============================================================================

import { useState, useEffect } from 'react';
import { Routes, Route, Navigate, useNavigate, useLocation, Outlet } from 'react-router-dom';
import { Plus, GitBranch, Home, Settings, HelpCircle, AlertCircle } from 'lucide-react';
import toast from 'react-hot-toast';
import { GitHealthService } from './services/gitService';

// Import des pages
import { DashboardPage } from './pages/Dashboard';
import { RepositoryListPage } from './pages/Repository/List';
import { RepositoryCreatePage } from './pages/Repository/Create';
import { RepositoryDetailPage } from './pages/Repository/Detail';

// ==----------------------------------------------------------------------------
// Composant Header
// ==----------------------------------------------------------------------------

function Header() {
  const navigate = useNavigate();

  return (
    <header className="header border-b border-gray-200 bg-white sticky top-0 z-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          {/* Logo */}
          <div className="flex items-center space-x-4">
            <button
              onClick={() => navigate('/')}
              className="flex items-center space-x-2 text-xl font-bold text-gray-900 hover:text-blue-600 transition-colors"
            >
              <span className="w-8 h-8 bg-blue-600 rounded flex items-center justify-center text-white font-bold text-sm">
                T
              </span>
              <span>Tardigrade-CI</span>
            </button>
          </div>

          {/* Navigation */}
          <nav className="hidden md:flex items-center space-x-1">
            <NavLink to="/">
              <Home className="w-4 h-4 mr-1" />
              Accueil
            </NavLink>
            <NavLink to="/repositories">
              <GitBranch className="w-4 h-4 mr-1" />
              Repositories
            </NavLink>
            <NavLink to="/settings">
              <Settings className="w-4 h-4 mr-1" />
              Paramètres
            </NavLink>
            <NavLink to="/help">
              <HelpCircle className="w-4 h-4 mr-1" />
              Aide
            </NavLink>
          </nav>

          {/* Actions */}
          <div className="flex items-center space-x-3">
            <button
              onClick={() => navigate('/repositories/create')}
              className="btn bg-blue-600 text-white px-4 py-2 text-sm font-medium hover:bg-blue-700 flex items-center space-x-2"
            >
              <Plus className="w-4 h-4" />
              <span className="hidden sm:inline">Nouveau</span>
            </button>
            <button
              onClick={() => toast.success('Notification de test !')}
              className="p-2 text-gray-500 hover:text-gray-700"
              title="Notifications"
            >
              <AlertCircle className="w-5 h-5" />
            </button>
          </div>
        </div>
      </div>
    </header>
  );
}

// ==----------------------------------------------------------------------------
// Composant NavLink
// ==----------------------------------------------------------------------------

interface NavLinkProps {
  to: string;
  children: React.ReactNode;
}

function NavLink({ to, children }: NavLinkProps) {
  const navigate = useNavigate();
  const location = useLocation();
  const isActive = location.pathname === to || location.pathname.startsWith(`${to}/`);

  return (
    <button
      onClick={() => navigate(to)}
      className={`px-3 py-2 text-sm font-medium rounded-md transition-colors flex items-center space-x-1 ${
        isActive
          ? 'bg-blue-50 text-blue-600'
          : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900'
      }`}
    >
      {children}
    </button>
  );
}

// ==----------------------------------------------------------------------------
// Composant Sidebar (optionnel - masqué pour le MVP)
// ==----------------------------------------------------------------------------

function Sidebar() {
  return null; // Masqué pour le MVP, sera implémenté plus tard
}

// ==----------------------------------------------------------------------------
// Composant Main Layout
// ==----------------------------------------------------------------------------

function MainLayout({ children }: { children: React.ReactNode }) {
  const [healthStatus, setHealthStatus] = useState<string | null>(null);

  // Vérifier la santé de l'API au chargement
  useEffect(() => {
    const checkHealth = async () => {
      try {
        await GitHealthService.healthCheck();
        setHealthStatus('healthy');
      } catch (error) {
        setHealthStatus('unhealthy');
        console.error('API Health Check Failed:', error);
      }
    };

    checkHealth();

    // Vérifier périodiquement
    const interval = setInterval(checkHealth, 30000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="min-h-screen bg-gray-50">
      <Header />
      <div className="flex">
        <Sidebar />
        <main className="flex-1 p-4 sm:p-6 lg:p-8">{children}</main>
      </div>

      {/* Indicateur de santé */}
      {healthStatus && (
        <div
          className={`fixed bottom-4 right-4 px-3 py-1 text-xs font-medium rounded-full ${
            healthStatus === 'healthy'
              ? 'bg-green-100 text-green-800'
              : 'bg-red-100 text-red-800'
          }`}
        >
          API: {healthStatus === 'healthy' ? '✓' : '✗'}
        </div>
      )}
    </div>
  );
}

// ==----------------------------------------------------------------------------
// Composant NotFound
// ==----------------------------------------------------------------------------

function NotFoundPage() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="text-center">
        <h1 className="text-6xl font-bold text-gray-300">404</h1>
        <p className="text-xl text-gray-600 mt-4">Page non trouvée</p>
        <p className="text-gray-500 mt-2">
          La page que vous cherchez n'existe pas ou a été déplacée.
        </p>
        <div className="mt-6 flex justify-center space-x-4">
          <button
            onClick={() => navigate('/')}
            className="btn bg-blue-600 text-white px-4 py-2 hover:bg-blue-700"
          >
            Retour à l'accueil
          </button>
          <button
            onClick={() => navigate('/repositories')}
            className="btn bg-gray-100 text-gray-700 px-4 py-2 hover:bg-gray-200"
          >
            Voir les repositories
          </button>
        </div>
      </div>
    </div>
  );
}

// ==----------------------------------------------------------------------------
// Routing principal
// ==----------------------------------------------------------------------------

function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<MainLayout><Outlet /></MainLayout>}>
        {/* Rediriger / vers /repositories */}
        <Route index element={<Navigate to="/repositories" replace />} />

        {/* Dashboard */}
        <Route path="dashboard" element={<DashboardPage />} />

        {/* Repositories */}
        <Route path="repositories" element={<RepositoryListPage />} />
        <Route path="repositories/create" element={<RepositoryCreatePage />} />
        <Route path="repositories/:id" element={<RepositoryDetailPage />} />

        {/* Paramètres */}
        <Route path="settings" element={<SettingsPage />} />

        {/* Aide */}
        <Route path="help" element={<HelpPage />} />

        {/* 404 */}
        <Route path="*" element={<NotFoundPage />} />
      </Route>
    </Routes>
  );
}

// ==----------------------------------------------------------------------------
// Pages temporaires
// ==----------------------------------------------------------------------------

function SettingsPage() {
  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-3xl font-bold text-gray-900 mb-6">Paramètres</h1>
      <div className="bg-white rounded-lg shadow p-6">
        <p className="text-gray-600">Les paramètres seront implémentés dans une version future.</p>
      </div>
    </div>
  );
}

function HelpPage() {
  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-3xl font-bold text-gray-900 mb-6">Aide</h1>
      <div className="bg-white rounded-lg shadow p-6">
        <p className="text-gray-600 mb-4">
          Bienvenue sur Tardigrade-CI, une plateforme DevOps modulaire open-source.
        </p>
        <p className="text-gray-600 mb-4">
          Ce projet est en cours de développement. La documentation complète
          sera disponible prochainement.
        </p>
        <div className="bg-gray-50 rounded p-4">
          <h3 className="font-semibold text-gray-900 mb-2">Ressources utiles :</h3>
          <ul className="space-y-1 text-gray-600">
            <li>
              <a
                href="https://rust-lang.org"
                target="_blank"
                rel="noopener noreferrer"
                className="text-blue-600 hover:underline"
              >
                Documentation Rust
              </a>
            </li>
            <li>
              <a
                href="https://react.dev"
                target="_blank"
                rel="noopener noreferrer"
                className="text-blue-600 hover:underline"
              >
                Documentation React
              </a>
            </li>
            <li>
              <a
                href="https://tailwindcss.com"
                target="_blank"
                rel="noopener noreferrer"
                className="text-blue-600 hover:underline"
              >
                Documentation Tailwind CSS
              </a>
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
}

// ==----------------------------------------------------------------------------
// Composant principal App
// ==----------------------------------------------------------------------------

export default function App() {
  return <AppRoutes />;
}
