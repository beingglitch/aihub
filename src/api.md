glitch@glitch:~$ 
    curl "https://api.openai.com/v1/responses"     
        -H "Content-Type: application/json"     
        -H "Authorization: Bearer $OPENAI_API_KEY"     
        -d '{
            "model": "gpt-5",
            "input": "what is rust"
        }'


    {
        "id": "resp_02a6cf253028183700690ee34c5b7c8196aa1386a3377663bd",
        "object": "response",
        "created_at": 1762583372,
        "status": "completed",
        "background": false,
        "billing": {
            "payer": "developer"
        },
        "error": null,
        "incomplete_details": null,
        "instructions": null,
        "max_output_tokens": null,
        "max_tool_calls": null,
        "model": "gpt-5-2025-08-07",
        "output": [
            {
            "id": "rs_02a6cf253028183700690ee34d4eb88196ad7ebd2b9b7070e2",
            "type": "reasoning",
            "summary": []
            },
            {
            "id": "msg_02a6cf253028183700690ee353c4948196b9279674fa8a641d",
            "type": "message",
            "status": "completed",
            "content": [
                {
                    "type": "output_text",
                    "annotations": [],
                    "logprobs": [],
                    "text": "\u201cRust\u201d can mean a few different things:\n\n- Rust (programming language): A fast, memory-safe systems language known for its ownership/borrowing model, zero-cost abstractions, no garbage collector, and great tooling (Cargo). Used for systems software, web backends, embedded, and performance-critical code.\n\n- Rust (corrosion): The reddish-brown iron oxides that form when iron or steel reacts with oxygen and moisture. It weakens metal; prevention includes coatings/paint, galvanization, stainless alloys, and cathodic protection.\n\n- Rust (plant disease): A group of fungal diseases that produce rust-colored pustules on leaves and stems, reducing crop yields; managed via resistant varieties and fungicides.\n\nWhich one do you mean?"
                }
            ],
            "role": "assistant"
            }
        ],
        "parallel_tool_calls": true,
        "previous_response_id": null,
        "prompt_cache_key": null,
        "prompt_cache_retention": null,
        "reasoning": {
            "effort": "medium",
            "summary": null
        },
        "safety_identifier": null,
        "service_tier": "default",
        "store": true,
        "temperature": 1.0,
        "text": {
            "format": {
            "type": "text"
            },
            "verbosity": "medium"
        },
        "tool_choice": "auto",
        "tools": [],
        "top_logprobs": 0,
        "top_p": 1.0,
        "truncation": "disabled",
        "usage": {
            "input_tokens": 9,
            "input_tokens_details": {
            "cached_tokens": 0
            },
            "output_tokens": 606,
            "output_tokens_details": {
            "reasoning_tokens": 448
            },
            "total_tokens": 615
        },
        "user": null,
        "metadata": {}
    }

