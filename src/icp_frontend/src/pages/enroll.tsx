// Enroll.tsx
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useAuth } from "@/contexts/AuthContext";
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Globe } from 'lucide-react';
import { toast } from "@/hooks/use-toast";
 import { profileActor } from '../icpConnect';// Link to the backend actor

 

interface Platform {
  name: string;
  username: string;
}

function Enroll() {
  const { user } = useAuth();
  const navigate = useNavigate();

  const [formData, setFormData] = useState({
    firstName: '',
    lastName: '',
  });

  const [platforms, setPlatforms] = useState<Platform[]>([
    { name: '', username: '' }
  ]);

  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handlePlatformChange = (index: number, field: keyof Platform, value: string) => {
    const updated = [...platforms];
    updated[index][field] = value;
    setPlatforms(updated);
  };

  const addPlatform = () => {
    setPlatforms([...platforms, { name: '', username: '' }]);
  };

  const removePlatform = (index: number) => {
    if (platforms.length > 1) {
      setPlatforms(platforms.filter((_, i) => i !== index));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      if (!formData.firstName || !formData.lastName) throw new Error('Missing name fields');
      if (platforms.some(p => !p.name || !p.username)) throw new Error('Fill all platform fields');

      const actor = await createUserActor();

      const result = await actor.enroll_user(user?.id || '', {
        firstName: formData.firstName,
        lastName: formData.lastName,
        platforms
      });

      toast({ title: "Enrolled", description: result, variant: "default" });
      navigate('/dashboard');

    } catch (err) {
      toast({
        title: "Enrollment Failed",
        description: err instanceof Error ? err.message : 'Unknown error',
        variant: "destructive",
      });
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex justify-center items-center min-h-screen">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle className="text-center text-2xl font-bold">Enroll</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-6">
            <div>
              <Label>First Name</Label>
              <Input name="firstName" value={formData.firstName} onChange={handleInputChange} required />
            </div>
            <div>
              <Label>Last Name</Label>
              <Input name="lastName" value={formData.lastName} onChange={handleInputChange} required />
            </div>
            <div className="space-y-4">
              <Label>Platform Accounts</Label>
              {platforms.map((p, i) => (
                <div key={i} className="flex gap-2">
                  <Input placeholder="Platform" value={p.name} onChange={(e) => handlePlatformChange(i, 'name', e.target.value)} />
                  <Input placeholder="Username" value={p.username} onChange={(e) => handlePlatformChange(i, 'username', e.target.value)} />
                  {platforms.length > 1 && (
                    <Button type="button" onClick={() => removePlatform(i)}>Remove</Button>
                  )}
                </div>
              ))}
              <Button type="button" onClick={addPlatform} variant="outline">Add Platform</Button>
            </div>
            <Button type="submit" disabled={isSubmitting} className="w-full">
              {isSubmitting ? "Submitting..." : "Enroll"}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}

export default Enroll;
