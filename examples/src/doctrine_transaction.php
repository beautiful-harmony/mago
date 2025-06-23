<?php
declare(strict_types=1);

namespace Mago\Examples;

use Doctrine\ORM\EntityManagerInterface;
use Exception;

class ExampleService
{
    private EntityManagerInterface $entityManager;
    
    public function __construct(EntityManagerInterface $entityManager)
    {
        $this->entityManager = $entityManager;
    }
    
    // ✅ Valid: beginTransaction() called outside try block
    public function validTransaction(): void
    {
        $this->entityManager->beginTransaction();
        try {
            $this->entityManager->persist(new \stdClass());
            $this->entityManager->flush();
            $this->entityManager->commit();
        } catch (Exception $e) {
            $this->entityManager->rollback();
            throw $e;
        }
    }
    
    // ✅ Valid: beginTransaction() called outside try block with getDoctrine()
    public function validTransactionWithGetManager(): void
    {
        $em = $this->getDoctrine()->getManager();
        $em->beginTransaction();
        try {
            // Database operations
            $em->commit();
        } catch (Exception $e) {
            $em->rollback();
        }
    }
    
    // ❌ Invalid: beginTransaction() called inside try block
    public function invalidTransactionInsideTry(): void
    {
        try {
            $this->entityManager->beginTransaction();
            $this->entityManager->persist(new \stdClass());
            $this->entityManager->flush();
            $this->entityManager->commit();
        } catch (Exception $e) {
            $this->entityManager->rollback();
            throw $e;
        }
    }
    
    // ❌ Invalid: beginTransaction() called inside try block with variable
    public function invalidTransactionWithVariable(): void
    {
        try {
            $em = $this->getDoctrine()->getManager();
            $em->beginTransaction();
            // Database operations
            $em->commit();
        } catch (Exception $e) {
            $em->rollback();
        }
    }
    
    private function getDoctrine()
    {
        return new class {
            public function getManager() {
                return new class {
                    public function beginTransaction() {}
                    public function commit() {}
                    public function rollback() {}
                    public function persist($entity) {}
                    public function flush() {}
                };
            }
        };
    }
}