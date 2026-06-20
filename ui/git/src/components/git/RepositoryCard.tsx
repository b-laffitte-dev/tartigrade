import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Repository } from '../../types/git';
import { Card, CardHeader, CardContent, CardFooter, Badge, Button } from '../common';
import { formatDate } from '../../utils/formatters';
import { PATHS } from '../../routes/paths';

export interface RepositoryCardProps {
  repository: Repository;
  onDelete?: (id: string) => void;
}

const RepositoryCard: React.FC<RepositoryCardProps> = ({ repository, onDelete }) => {
  const navigate = useNavigate();

  const handleViewClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    navigate(PATHS.repository.detail(repository.id));
  };

  const handleDeleteClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    onDelete?.(repository.id);
  };

  return (
    <Card
      variant="bordered"
      className="hover:shadow-lg transition-shadow duration-200 cursor-pointer"
      onClick={handleViewClick}
    >
      <CardHeader>
        <div className="flex justify-between items-start">
          <div className="flex-1 min-w-0">
            <h3 className="text-lg font-semibold text-secondary-900 truncate">
              {repository.name}
            </h3>
            {repository.description && (
              <p className="text-sm text-secondary-500 mt-1 line-clamp-2">
                {repository.description}
              </p>
            )}
          </div>
          <Badge variant={repository.isPrivate ? 'default' : 'success'}>{
            repository.isPrivate ? 'Privé' : 'Public'
          }</Badge>
        </div>
      </CardHeader>

      <CardContent className="mt-4">
        <span className="text-sm text-secondary-500">
          Branche par défaut: {repository.defaultBranch}
        </span>
      </CardContent>

      <CardFooter className="mt-4">
        <div className="flex justify-between items-center">
          <span className="text-xs text-secondary-400">
            Créé le {formatDate(repository.createdAt)}
          </span>
          <div className="flex gap-2">
            <Button variant="outline" size="sm" onClick={handleViewClick}>
              Voir
            </Button>
            {onDelete && (
              <Button
                variant="destructive"
                size="sm"
                onClick={handleDeleteClick}
              >
                Supprimer
              </Button>
            )}
          </div>
        </div>
      </CardFooter>
    </Card>
  );
};

export { RepositoryCard };
