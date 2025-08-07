import { useEffect, useState } from 'react'
import './App.css'
import Bar from './Bar.jsx'

const apiUrl = 'http://localhost:3000';

const fundingAmount = 100_000;
const mintAmount = 10_000;

const states = {
  Funding: 'funding',
  Submitted: 'submitted',
}

function App() {
  const [rawTx, setRawTx] = useState('');
  const [state, setState] = useState(states.Funding);
  const [note, setNote] = useState(undefined);

  const [fundingAddress, setFundingAddress] = useState('');

  useState(() => {
    const fetchAddress = async () => {
      try {
        const response = await fetch(`${apiUrl}/address`);
        const data = await response.json();
        setFundingAddress(data.address);
      } catch (error) {
        console.error('Failed to fetch funding address:', error);
      }
    };
    
    fetchAddress();
  }, []);

  const [ourAmount, setOurAmount] = useState(fundingAmount);

  return (
    <div style={{ 
      padding: '20px', 
      textAlign: 'center',
      display: 'flex',
      flexDirection: 'column',
    }}>
      {state === states.Funding && (
        <>
              <h2>Funding address</h2>
      <textarea 
        disabled={true}
        value={fundingAddress}
        placeholder=""
        style={{
          width: '100%',
          maxWidth: '600px',
          padding: '15px',
          fontSize: '16px',
          border: '2px solid #ccc',
          borderRadius: '8px',
        }}
      />
              <div style={{ height: '50px' }}></div>
      <h2>Enter Transaction</h2>
      <textarea 
        value={rawTx}
        onChange={(e) => setRawTx(e.target.value)}
        placeholder="Enter raw transaction here..."
        style={{
          width: '500px',
          maxWidth: '600px',
          height: '200px',
          fontSize: '16px',
          padding: '15px',
          border: '2px solid #ccc',
          borderRadius: '8px',
          resize: 'vertical'
        }}
      />
      <div style={{ height: '30px' }}></div>
      <button 
        onClick={async () => {
          try {
            const response = await fetch(`${apiUrl}/tx`, {
              method: 'POST',
              headers: {
                'Content-Type': 'application/json',
              },
              body: JSON.stringify({ tx: rawTx }),
            });
            
            if (response.ok) {
              const result = await response.json();
              console.log('Transaction submitted:', result);
              setState(states.Submitted);
            } else {
              console.error('Failed to submit transaction');
            }
          } catch (error) {
            console.error('Error submitting transaction:', error);
          }
        }}
        style={{
          padding: '12px 24px',
          fontSize: '16px',
          backgroundColor: '#007bff',
          color: 'white',
          border: 'none',
          borderRadius: '8px',
          cursor: 'pointer',
          maxWidth: '200px',
          alignSelf: 'center'
        }}
        onMouseOver={(e) => e.target.style.backgroundColor = '#0056b3'}
        onMouseOut={(e) => e.target.style.backgroundColor = '#007bff'}
      >
        Submit Transaction
      </button>
        </>
      )}
      {state === states.Submitted && (
        <>
          <h2>Create ecash</h2>
      <Bar totalAmount={fundingAmount} ourAmount={ourAmount} />
                <button 
        onClick={async () => {
          const response = await fetch(`${apiUrl}/token`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({ amount: mintAmount }),
          });
          
          const data = await response.json();
          setNote(data.token);
          setOurAmount(ourAmount - mintAmount);
        }}
        style={{
          padding: '12px 24px',
          fontSize: '16px',
          backgroundColor: '#007bff',
          color: 'white',
          border: 'none',
          borderRadius: '8px',
          cursor: 'pointer',
          maxWidth: '200px',
          alignSelf: 'center'
        }}
        onMouseOver={(e) => e.target.style.backgroundColor = '#0056b3'}
        onMouseOut={(e) => e.target.style.backgroundColor = '#007bff'}
      >
        {mintAmount} sats
      </button>
      <div style={{ height: '30px' }}></div>

      {note !== undefined && (
      <>
          <h2>Note</h2>
          <textarea 
            disabled={true}
            value={note}
            style={{
              width: '100%',
              maxWidth: '600px',
              height: '150px',
              padding: '15px',
              fontSize: '16px',
              border: '2px solid #ccc',
              borderRadius: '8px',
              alignSelf: 'center',
              resize: 'vertical'
            }}
          />
          <button 
            onClick={() => {
              navigator.clipboard.writeText(note);
              // Optional: Add visual feedback
              const button = event.target;
              const originalText = button.textContent;
              button.textContent = 'Copied!';
              setTimeout(() => {
                button.textContent = originalText;
              }, 1000);
            }}
            style={{
              marginTop: '10px',
              padding: '8px 16px',
              fontSize: '14px',
              backgroundColor: '#28a745',
              color: 'white',
              border: 'none',
              borderRadius: '6px',
              cursor: 'pointer',
              alignSelf: 'center'
            }}
            onMouseOver={(e) => e.target.style.backgroundColor = '#218838'}
            onMouseOut={(e) => e.target.style.backgroundColor = '#28a745'}
          >
            Copy Note
          </button>
        </>
      )}
        </>
      )}
    </div>
  )
}

export default App
