<?php

declare(strict_types=1);

namespace Mago\Examples;

class UserRepository extends EntityRepository
{
    public function getByEmail(string $email): ?User
    {
        return $this->findOneBy(['email' => $email]);
    }

    public function getUserById(int $id): User
    {
        return $this->find($id);
    }
}