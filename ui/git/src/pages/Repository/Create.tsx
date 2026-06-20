import React from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { useNavigate } from 'react-router-dom';
import { PATHS } from '../../routes/paths';
import { useCreateRepository } from '../../hooks';
import { Button, Input, Loading } from '../../components/common';
import toast from 'react-hot-toast';

const createRepositorySchema = z.object({
  name: z
    .string()
    .min(1, 'Le nom est obligatoire')
    .max(100, 'Le nom ne peut pas dépasser 100 caractères')
    .regex(
      /^[a-zA-Z0-9-_.]+$/,
      'Le nom ne peut contenir que des caractères alphanumériques, des tirets, des underscores et des points'
    ),
  description: z
    .string()
    .max(500, 'La description ne peut pas dépasser 500 caractères')
    .optional(),
  isPrivate: z.boolean().default(false),
  defaultBranch: z
    .string()
    .min(1, 'La branche par défaut est obligatoire')
    .default('main'),
});

type CreateRepositoryFormData = z.infer<typeof createRepositorySchema>;

const RepositoryCreate: React.FC = () => {
  const navigate = useNavigate();
  const [createRepository, { loading }] = useCreateRepository();

  const form = useForm<CreateRepositoryFormData>({
    resolver: zodResolver(createRepositorySchema),
    defaultValues: {
      name: '',
      description: '',
      isPrivate: false,
      defaultBranch: 'main',
    },
  });

  const onSubmit = async (data: CreateRepositoryFormData) => {
    try {
      await createRepository({
        variables: {
          input: {
            name: data.name,
            description: data.description || undefined,
            isPrivate: data.isPrivate,
            defaultBranch: data.defaultBranch,
          },
        },
      });
      toast.success('Repository créé avec succès !');
      navigate(PATHS.repository.list);
    } catch (err) {
      console.error('Échec de la création:', err);
      toast.error('Échec de la création du repository');
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-secondary-900">Créer un Repository</h1>
        <p className="text-secondary-500 mt-1">
          Créez un nouveau repository pour votre code.
        </p>
      </div>

      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-secondary-700 mb-1">
              Nom *
            </label>
            <Input
              {...form.register('name')}
              placeholder="mon-projet"
              error={form.formState.errors.name?.message}
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-secondary-700 mb-1">
              Description
            </label>
            <Input
              {...form.register('description')}
              placeholder="Description de votre repository..."
              error={form.formState.errors.description?.message}
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-secondary-700 mb-1">
              Branche par défaut
            </label>
            <Input
              {...form.register('defaultBranch')}
              placeholder="main"
              error={form.formState.errors.defaultBranch?.message}
            />
          </div>

          <div className="flex items-center gap-2">
            <input
              type="checkbox"
              id="isPrivate"
              {...form.register('isPrivate')}
              className="w-4 h-4 rounded border-secondary-300 text-primary-600 focus:ring-primary-500"
            />
            <label htmlFor="isPrivate" className="text-sm text-secondary-700">
              Repository privé
            </label>
          </div>
        </div>

        <div className="flex gap-4">
          <Button
            type="button"
            variant="outline"
            onClick={() => navigate(PATHS.repository.list)}
          >
            Annuler
          </Button>
          <Button type="submit" disabled={loading}>
            {loading ? <Loading spinner /> : 'Créer le Repository'}
          </Button>
        </div>
      </form>
    </div>
  );
};

export { RepositoryCreate };
