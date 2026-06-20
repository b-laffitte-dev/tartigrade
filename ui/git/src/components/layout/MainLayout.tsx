import React from 'react';
import { Outlet, Link } from 'react-router-dom';
import { PATHS } from '../../routes/paths';

const MainLayout: React.FC = () => {
  return (
    <div className="min-h-screen bg-secondary-50">
      {/* Header */}
      <header className="bg-white border-b border-secondary-200 sticky top-0 z-10">
        <div className="container mx-auto px-4 py-4">
          <div className="flex justify-between items-center">
            <Link to={PATHS.home} className="flex items-center gap-2">
              <div className="w-8 h-8 bg-primary-600 rounded-lg flex items-center justify-center">
                <span className="text-white font-bold text-sm">T</span>
              </div>
              <span className="text-xl font-bold text-secondary-900">Tardigrade-CI</span>
            </Link>
            <nav className="flex items-center gap-6">
              <Link
                to={PATHS.home}
                className="text-secondary-600 hover:text-primary-600 transition-colors"
              >
                Dashboard
              </Link>
              <Link
                to={PATHS.repository.list}
                className="text-secondary-600 hover:text-primary-600 transition-colors"
              >
                Repositories
              </Link>
            </nav>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        <Outlet />
      </main>

      {/* Footer */}
      <footer className="bg-white border-t border-secondary-200 py-6">
        <div className="container mx-auto px-4 text-center text-secondary-500">
          <p>© {new Date().getFullYear()} Tardigrade-CI. Tous droits réservés.</p>
        </div>
      </footer>
    </div>
  );
};

export { MainLayout };
