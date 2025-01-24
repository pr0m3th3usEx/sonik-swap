import { Button } from './ui/button';

const status: 'connected' | 'not_linked' = 'connected';

export default function AccountConnectionManagerListItem() {
  return (
    <>
      <p className="col-span-3">Deezer</p>
      {status === 'connected' ? (
        <Button variant="link" className="text-red-400">
          Revoke
        </Button>
      ) : (
        <Button>Connect</Button>
      )}
    </>
  );
}
