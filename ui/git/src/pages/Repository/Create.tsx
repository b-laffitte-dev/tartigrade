// =============================================================================
// Tardigrade-CI Git Module - Create Repository Page
// =============================================================================

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useNavigate } from 'react-router-dom';
import { GitBranch, Check, AlertCircle } from 'lucide-react';
import { createRepositorySchema, CreateRepositorySchema } from '../../types/git';
import { useRepositories } from '../../hooks/useRepositories';

// ==----------------------------------------------------------------------------
// Composant Create Repository Page
// ==----------------------------------------------------------------------------

export function RepositoryCreatePage() {
  const navigate = useNavigate();
  const { createRepository } = useRepositories();

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
    setValue,
    watch,
  } = useForm<CreateRepositorySchema>({
    resolver: zodResolver(createRepositorySchema),
    defaultValues: {
      name: '',
      description: '',
      isPrivate: false,
      defaultBranch: 'main',
    },
  });

  const isPrivate = watch('isPrivate');
  const name = watch('name');

  const onSubmit = async (data: CreateRepositorySchema) => {
    try {
      await createRepository(data);
    } catch (error) {
      // Les erreurs sont gérées dans le hook
      console.error('Error creating repository:', error);
    }
  };

  const handleCancel = () => {
    navigate('/repositories');
  };

  return (
    <div className="max-w-2xl mx-auto">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center space-x-3 mb-2">
          <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
            <GitBranch className="w-5 h-5 text-blue-600" />
          </div>
          <div>
            <h1 className="text-3xl font-bold text-gray-900">Créer un Repository</h1>
            <p className="text-gray-500">
              Créez un nouveau repository pour versionner votre code
            </p>
          </div>
        </div>
      </div>

      {/* Formulaire */}
      <form onSubmit={handleSubmit(onSubmit)} className="bg-white rounded-lg shadow p-6">
        {/* Nom du repository */}
        <div className="mb-6">
          <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-2">
            Nom du repository *
          </label>
          <div className="relative">
            <input
              id="name"
              type="text"
              placeholder="mon-projet"
              className={`input w-full px-3 py-2 ${
                errors.name ? 'border-red-300 focus:border-red-500' : ''
              }`}
              {...register('name')}
              autoFocus
            />
            {name && !errors.name && (
              <div className="absolute right-3 top-1/2 -translate-y-1/2 text-green-500">
                <Check className="w-5 h-5" />
              </div>
            )}
          </div>
          {errors.name && (
            <p className="mt-1 text-sm text-red-600 flex items-center">
              <AlertCircle className="w-4 h-4 mr-1" />
              {errors.name.message}
            </p>
          )}
          <p className="text-xs text-gray-500 mt-1">
            Utilisez des caractères alphanumériques, -, _ ou .
          </p>
        </div>

        {/* Description */}
        <div className="mb-6">
          <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-2">
            Description
          </label>
          <textarea
            id="description"
            placeholder="Description de votre repository..."
            rows={3}
            className="input w-full px-3 py-2 resize-none"
            {...register('description')}
          />
          <p className="text-xs text-gray-500 mt-1">
            {watch('description')?.length || 0}/500 caractères
          </p>
        </div>

        {/* Branche par défaut */}
        <div className="mb-6">
          <label htmlFor="defaultBranch" className="block text-sm font-medium text-gray-700 mb-2">
            Branche par défaut
          </label>
          <div className="relative">
            <input
              id="defaultBranch"
              type="text"
              placeholder="main"
              className="input w-full px-3 py-2"
              {...register('defaultBranch')}
            />
          </div>
          {errors.defaultBranch && (
            <p className="mt-1 text-sm text-red-600">{errors.defaultBranch.message}</p>
          )}
        </div>

        {/* Visibilité */}
        <div className="mb-6">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Visibilité
          </label>
          <div className="flex space-x-4">
            <label className="flex items-center space-x-2 cursor-pointer">
              <input
                type="radio"
                value="public"
                checked={!isPrivate}
                onChange={() => setValue('isPrivate', false)}
                className="w-4 h-4 text-blue-600 border-gray-300 focus:ring-blue-500"
              />
              <span className="text-sm font-medium">Public</span>
              <span className="text-xs text-gray-500">
                Tout le monde peut voir ce repository
              </span>
            </label>
            <label className="flex items-center space-x-2 cursor-pointer">
              <input
                type="radio"
                value="private"
                checked={isPrivate}
                onChange={() => setValue('isPrivate', true)}
                className="w-4 h-4 text-blue-600 border-gray-300 focus:ring-blue-500"
              />
              <span className="text-sm font-medium">Privé</span>
              <span className="text-xs text-gray-500">
                Seuls vous et vos collaborateurs pouvez voir ce repository
              </span>
            </label>
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center justify-end space-x-3 pt-6 border-t border-gray-200">
          <button
            type="button"
            onClick={handleCancel}
            className="btn bg-gray-100 text-gray-700 px-4 py-2 text-sm font-medium hover:bg-gray-200"
          >
            Annuler
          </button>
          <button
            type="submit"
            disabled={isSubmitting}
            className="btn bg-blue-600 text-white px-4 py-2 text-sm font-medium hover:bg-blue-700 disabled:opacity-60 disabled:cursor-not-allowed flex items-center space-x-2"
          >
            {isSubmitting ? (
              <>
                <span className="spinner w-4 h-4" />
                <span>Création...</span>
              </>
            ) : (
              <>
                <Check className="w-4 h-4" />
                <span>Créer le repository</span>
              </>
            )}
          </button>
        </div>
      </form>
    </div>
  );
}
